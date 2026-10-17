//! Interpreter for semantic_ir expressions and statements.
//!
//! This module provides an evaluator for `Expr` and an executor for `Stmt`
//! that operates on the `CpuState` architectural model.

use crate::expr::{BinOp, Expr, UnaryOp};
use crate::primitives::{AddWithCarry, AtomicRMWOp, Condition, ConditionHolds};
use crate::shared::enums::{ConditionFlags, PhysMemRetStatus, Shareability, SecurityState, ExceptionLevel, AccessType};
use crate::shared::registers::{CpuState, GeneralRegisters, PState};
use crate::shared::system::{AccessDescriptor, FaultRecord, MemorySystem, aarch64_translate_address, check_sp_alignment};
use crate::stmt::Stmt;
use crate::types::{BitWidth, Flag, MemSize, Reg, SpecialReg, ValueType, Width};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InterpreterError {
    #[error("Unimplemented expression: {0}")]
    UnimplementedExpr(String),
    #[error("Unimplemented statement: {0}")]
    UnimplementedStmt(String),
    #[error("Invalid register: {0}")]
    InvalidRegister(String),
    #[error("Memory fault: {0:?}")]
    MemoryFault(FaultRecord),
    #[error("Division by zero")]
    DivByZero,
    #[error("Invalid bit width: {0}")]
    InvalidBitWidth(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Branch taken to {target:#x}")]
    BranchTaken { target: u64 },
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// Interpreter for executing semantic_ir on CpuState.
#[derive(Debug, Clone)]
pub struct Interpreter {
    /// Current CPU state
    pub state: CpuState,
    /// Whether a branch was taken (for B/B.cond/CBZ/etc.)
    pub branch_taken: Option<u64>,
}

impl Interpreter {
    /// Create a new interpreter with default CPU state (EL1)
    pub fn new() -> Self {
        Self {
            state: CpuState::new(),
            branch_taken: None,
        }
    }

    /// Create interpreter with specific exception level
    pub fn with_el(el: crate::shared::enums::ExceptionLevel) -> Self {
        Self {
            state: CpuState::new().with_el(el),
            branch_taken: None,
        }
    }

    /// Execute a full instruction (sequence of statements)
    pub fn execute_instruction(&mut self, instr: &crate::instruction::Instruction) -> InterpreterResult<()> {
        self.branch_taken = None;
        for stmt in &instr.semantics {
            self.execute_stmt(stmt)?;
            if self.branch_taken.is_some() {
                // Branch taken - stop executing remaining statements
                break;
            }
        }
        Ok(())
    }

    /// Execute a single statement
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> InterpreterResult<()> {
        match stmt {
            Stmt::WriteReg { reg, value } => {
                let val = self.eval_expr(value)?;
                self.write_reg(reg, &val);
                Ok(())
            }
            Stmt::WriteSP { value, width: _ } => {
                let val = self.eval_expr(value)?;
                self.write_sp(&val);
                Ok(())
            }
            Stmt::WriteFlag { flag, value } => {
                let val = self.eval_expr(value)?;
                let bit = self.expr_to_bool(&val)?;
                self.write_flag(flag, bit);
                Ok(())
            }
            Stmt::Branch { target } => {
                let addr = self.eval_expr(target)?;
                let target_addr = self.expr_to_u64(&addr)?;
                self.branch_taken = Some(target_addr);
                self.state.gpr.write_pc(target_addr);
                Err(InterpreterError::BranchTaken { target: target_addr })
            }
            Stmt::BranchConditional { condition, target } => {
                let cond_val = self.eval_expr(condition)?;
                let taken = self.expr_to_bool(&cond_val)?;
                if taken {
                    let addr = self.eval_expr(target)?;
                    let target_addr = self.expr_to_u64(&addr)?;
                    self.branch_taken = Some(target_addr);
                    self.state.gpr.write_pc(target_addr);
                    Err(InterpreterError::BranchTaken { target: target_addr })
                } else {
                    Ok(())
                }
            }
            Stmt::If { condition, then_body, else_body } => {
                let cond_val = self.eval_expr(condition)?;
                let taken = self.expr_to_bool(&cond_val)?;
                let body = if taken { then_body } else { else_body };
                for stmt in body {
                    self.execute_stmt(stmt)?;
                    if self.branch_taken.is_some() {
                        break;
                    }
                }
                Ok(())
            }
            Stmt::Let { name, value } => {
                // For interpreter, we don't need to track local variables
                // They're only used during expression evaluation
                let _ = self.eval_expr(value)?;
                Ok(())
            }
            Stmt::Assign { name, value } => {
                let _ = self.eval_expr(value)?;
                Ok(())
            }
            Stmt::Discard { value } => {
                let _ = self.eval_expr(value)?;
                Ok(())
            }
            Stmt::While { condition, body, max_iterations } => {
                let mut iter = 0;
                let max_iter = max_iterations.unwrap_or(1000);
                loop {
                    if iter >= max_iter {
                        return Err(InterpreterError::Unsupported("Max iterations exceeded in While".into()));
                    }
                    let cond_val = self.eval_expr(condition)?;
                    let taken = self.expr_to_bool(&cond_val)?;
                    if !taken {
                        break;
                    }
                    for stmt in body {
                        self.execute_stmt(stmt)?;
                        if self.branch_taken.is_some() {
                            return Err(InterpreterError::Unsupported("Branch in While loop not supported".into()));
                        }
                    }
                    iter += 1;
                }
                Ok(())
            }
            Stmt::Return { value } => {
                if let Some(v) = value {
                    let _ = self.eval_expr(v)?;
                }
                // Return just stops execution - handled by caller
                Ok(())
            }
            Stmt::Match { scrutinee, arms } => {
                let scrutinee_val = self.eval_expr(scrutinee)?;
                for arm in arms {
                    if self.match_pattern(&arm.patterns, &scrutinee_val)? {
                        if let Some(guard) = &arm.guard {
                            let guard_val = self.eval_expr(guard)?;
                            if !self.expr_to_bool(&guard_val)? {
                                continue;
                            }
                        }
                        for stmt in &arm.body {
                            self.execute_stmt(stmt)?;
                            if self.branch_taken.is_some() {
                                break;
                            }
                        }
                        return Ok(());
                    }
                }
                Ok(())
            }
            Stmt::AssignPlace { place, value } => {
                // This is complex - for now just evaluate the value
                let _ = self.eval_expr(value)?;
                // TODO: Implement place assignment (register slices, etc.)
                Ok(())
            }
            Stmt::RaiseException(exc) => {
                Err(InterpreterError::Unsupported(format!("RaiseException: {:?}", exc)))
            }
            Stmt::MemWrite { address, value, size, access: _, ordering: _ } => {
                let addr = self.eval_expr(address)?;
                let val = self.eval_expr(value)?;
                let sz: MemSize = *size;
                self.memory_write(addr, val, sz)?;
                Ok(())
            }
            Stmt::MemRead { address, dest, size, access: _, ordering: _ } => {
                let addr = self.eval_expr(address)?;
                let sz: MemSize = *size;
                let val = self.memory_read(addr, sz)?;
                self.write_reg(dest, &val);
                Ok(())
            }
            Stmt::WriteSpecial { reg, value, width: _ } => {
                let val = self.eval_expr(value)?;
                self.write_special(reg, &val);
                Ok(())
            }
            Stmt::WritePredReg { reg: _, value, pl: _ } => {
                let _ = self.eval_expr(value)?;
                Ok(())
            }
            Stmt::WriteScalableVecReg { reg: _, value, vl: _ } => {
                let _ = self.eval_expr(value)?;
                Ok(())
            }
            Stmt::WriteIndexedReg { index, value, width: _ } => {
                let _ = self.eval_expr(index)?;
                let _ = self.eval_expr(value)?;
                Ok(())
            }
        }
    }

    /// Evaluate an expression to a u128 value
    pub fn eval_expr(&mut self, expr: &Expr) -> InterpreterResult<u128> {
        match expr {
            Expr::Const { value, ty: _ } => Ok(*value),
            Expr::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Expr::ReadReg { reg, width: _ } => Ok(self.read_reg(reg)),
            Expr::ReadSP { width: _ } => Ok(self.state.gpr.read_sp() as u128),
            Expr::ReadSpecial { reg, width: _ } => Ok(self.read_special(reg)),
            Expr::ReadFlag { flag } => Ok(if self.read_flag(flag) { 1 } else { 0 }),
            Expr::Binary { op, left, right, ty: _ } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binop(op, l, r)
            }
            Expr::Unary { op, value, ty: _ } => {
                let v = self.eval_expr(value)?;
                self.eval_unary_op(op, v)
            }
            Expr::Select { condition, when_true, when_false, ty: _ } => {
                let cond = self.eval_expr(condition)?;
                let taken = cond != 0;
                if taken {
                    self.eval_expr(when_true)
                } else {
                    self.eval_expr(when_false)
                }
            }
            Expr::ConditionHolds { condition } => {
                let cond = match *condition {
                    crate::primitives::Condition::Eq => crate::shared::enums::Condition::EQ,
                    crate::primitives::Condition::Ne => crate::shared::enums::Condition::NE,
                    crate::primitives::Condition::Cs => crate::shared::enums::Condition::CS,
                    crate::primitives::Condition::Cc => crate::shared::enums::Condition::CC,
                    crate::primitives::Condition::Mi => crate::shared::enums::Condition::MI,
                    crate::primitives::Condition::Pl => crate::shared::enums::Condition::PL,
                    crate::primitives::Condition::Vs => crate::shared::enums::Condition::VS,
                    crate::primitives::Condition::Vc => crate::shared::enums::Condition::VC,
                    crate::primitives::Condition::Hi => crate::shared::enums::Condition::HI,
                    crate::primitives::Condition::Ls => crate::shared::enums::Condition::LS,
                    crate::primitives::Condition::Ge => crate::shared::enums::Condition::GE,
                    crate::primitives::Condition::Lt => crate::shared::enums::Condition::LT,
                    crate::primitives::Condition::Gt => crate::shared::enums::Condition::GT,
                    crate::primitives::Condition::Le => crate::shared::enums::Condition::LE,
                    crate::primitives::Condition::Al => crate::shared::enums::Condition::AL,
                    crate::primitives::Condition::Nv => crate::shared::enums::Condition::NV,
                };
                Ok(if self.state.pstate.condition_holds(cond) { 1u128 } else { 0u128 })
            }
            Expr::AddWithCarry { left, right, carry, width } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                let c = self.eval_expr(carry)? != 0;
                let bw = *width;
                let (result, flags) = self.add_with_carry(l, r, c, bw)?;
                // Return tuple as packed value: result | (flags as u128) << 64
                // For interpreter, we'll just return result; flags are handled via WriteFlag statements
                Ok(result)
            }
            Expr::TupleElement { value, index, ty: _ } => {
                let val = self.eval_expr(value)?;
                match index {
                    0 => Ok(val), // result
                    1 => {
                        // NZCV flags - this should not happen in correct lowering
                        // (flags are extracted via nzcv_writeback -> WriteFlag)
                        Ok(0)
                    }
                    _ => Err(InterpreterError::TypeError(format!("Invalid tuple index: {}", index))),
                }
            }
            Expr::ZeroExtend { value, to } => {
                let v = self.eval_expr(value)?;
                Ok(v & ((1u128 << to.bits()) - 1))
            }
            Expr::SignExtend { value, to } => {
                let v = self.eval_expr(value)?;
                let bits = to.bits();
                if bits == 128 {
                    Ok(v as i128 as u128)
                } else {
                    // Determine source width from the value expression
                    let src_bits = match value.as_ref() {
                        Expr::Const { ty, .. } => ty.bits(),
                        // For other expressions, default to target width
                        _ => bits,
                    };
                    
                    let sign_bit = (v >> (src_bits - 1)) & 1;
                    let mask = if bits == 128 { !0u128 } else { (1u128 << bits) - 1 };
                    if sign_bit == 1 {
                        // Sign extend: replicate sign bit to all higher bits up to target width
                        let ext_mask = (!0u128 << src_bits) & mask;
                        Ok((v | ext_mask) & mask)
                    } else {
                        Ok(v & mask)
                    }
                }
            }
            Expr::BitTest { value, bit } => {
                let v = self.eval_expr(value)?;
                Ok(((v >> *bit) & 1) as u128)
            }
            Expr::Extract { value, lsb, width } => {
                let v = self.eval_expr(value)?;
                let mask = if width.bits() == 128 {
                    !0u128
                } else {
                    (1u128 << width.bits()) - 1
                };
                Ok((v >> *lsb) & mask)
            }
            Expr::Concat { high, low, width: _ } => {
                let h = self.eval_expr(high)?;
                let l = self.eval_expr(low)?;
                // Assume low is 64 bits for now
                Ok((h << 64) | l)
            }
            Expr::Replicate { value, count, element_width } => {
                let v = self.eval_expr(value)?;
                let mut result = 0u128;
                let elem_bits = element_width.bits();
                for i in 0..*count {
                    result |= v << (i * elem_bits);
                }
                Ok(result)
            }
            Expr::Shift { kind, value, amount, ty: _ } => {
                let v = self.eval_expr(value)?;
                let a = self.eval_expr(amount)?;
                self.eval_shift(kind, v, a)
            }
            Expr::Extend { kind, value, to } => {
                let v = self.eval_expr(value)?;
                self.eval_extend(kind, v, *to)
            }
            Expr::ExtendReg { register, kind, shift, to } => {
                // Simplified: read the register and extend
                let reg_val = match &**register {
                    Expr::ReadReg { reg, .. } => self.read_reg(reg),
                    Expr::Const { value, .. } => *value,
                    _ => return Err(InterpreterError::UnimplementedExpr("ExtendReg with complex register".into())),
                };
                let shift_val = self.eval_expr(shift)?;
                self.eval_extend_reg(kind, reg_val, shift_val, *to)
            }
            Expr::If { cond, then_branch, else_branch } => {
                let c = self.eval_expr(cond)?;
                if c != 0 {
                    self.eval_expr(then_branch)
                } else {
                    self.eval_expr(else_branch)
                }
            }
            Expr::Call { func, args } => {
                // Handle intrinsic calls
                self.eval_call(func, args)
            }
            Expr::FieldAccess { base, field } => {
                // Handle PSTATE field access
                if field == "PSTATE" {
                    // Return a dummy value - actual field access happens via ReadFlag
                    Ok(0)
                } else {
                    Err(InterpreterError::UnimplementedExpr(format!("FieldAccess: {}", field)))
                }
            }
            Expr::Slice { value, lsb, msb } => {
                let v = self.eval_expr(value)?;
                let width = *msb - *lsb + 1;
                let mask = if width >= 128 { !0u128 } else { (1u128 << width) - 1 };
                Ok((v >> *lsb) & mask)
            }
            Expr::GenericPrim { name, type_args, args } => {
                self.eval_generic_prim(name, type_args, args)
            }
            Expr::ChkFeat { value } => {
                let _ = self.eval_expr(value)?;
                Ok(1) // Assume feature present
            }
            Expr::Crc32 { acc, value, data_width, crc32c } => {
                let a = self.eval_expr(acc)?;
                let v = self.eval_expr(value)?;
                // Simplified CRC32 - just return accumulator
                Ok(a)
            }
            Expr::ReadPredReg { reg: _, pl: _ } => Ok(0),
            Expr::ReadScalableVecReg { reg: _, vl: _ } => Ok(0),
            Expr::WritePredReg { .. } | Expr::WriteScalableVecReg { .. } => Ok(0),
            Expr::PredicateMap { .. } | Expr::PredicateMap2 { .. } | Expr::PredicateReduce { .. } => Ok(0),
            Expr::ScalableVectorMap { .. } | Expr::ScalableVectorMap2 { .. } | Expr::ScalableVectorMap3 { .. } => Ok(0),
            Expr::ScalableVectorReduce { .. } | Expr::ScalableVectorPairwise { .. } => Ok(0),
            Expr::SVEGatherScatter { .. } | Expr::SVESpecial { .. } => Ok(0),
            Expr::VectorMap { .. } | Expr::VectorMap2 { .. } => Ok(0),
            Expr::VectorReduce { .. } | Expr::VectorPairwise { .. } | Expr::VectorPerm { .. } => Ok(0),
            Expr::VectorNarrowing { .. } | Expr::VectorDot { .. } => Ok(0),
            Expr::VectorDeinterleave { .. } | Expr::VectorInterleave { .. } => Ok(0),
            Expr::MemRead { address, size } => {
                let addr = self.eval_expr(address)?;
                self.memory_read(addr, *size)
            }
            Expr::MemAtomic { address, expected, new_value, size, ordering: _ } => {
                let addr = self.eval_expr(address)?;
                let exp = self.eval_expr(expected)?;
                let new_val = self.eval_expr(new_value)?;
                let sz: MemSize = *size;
                
                // Perform atomic compare-and-swap
                let old_val = self.memory_read(addr, sz)?;
                if old_val == exp {
                    self.memory_write(addr, new_val, sz)?;
                }
                Ok(old_val)
            }
            Expr::MemAtomicRMW { op, address, value, size, ordering: _ } => {
                let addr = self.eval_expr(address)?;
                let val = self.eval_expr(value)?;
                let sz: MemSize = *size;
                
                // Perform atomic read-modify-write
                let old_val = self.memory_read(addr, sz)?;
                let new_val = self.eval_atomic_rmw(*op, old_val, val, sz)?;
                self.memory_write(addr, new_val, sz)?;
                Ok(old_val)
            }
            _ => Err(InterpreterError::UnimplementedExpr(format!("{:?}", expr))),
        }
    }

    fn eval_binop(&self, op: &BinOp, l: u128, r: u128) -> InterpreterResult<u128> {
        use BinOp::*;
        Ok(match op {
            Add => l.wrapping_add(r),
            Sub => l.wrapping_sub(r),
            And => l & r,
            Or => l | r,
            Xor => l ^ r,
            Mul => l.wrapping_mul(r),
            Shl => l << (r as u32),
            Lsr => l >> (r as u32),
            Asr => {
                // Arithmetic shift right - sign extend
                let bits = 128; // TODO: use actual width
                if l & (1u128 << (bits - 1)) != 0 {
                    // Sign bit set - fill with 1s
                    let shift = r as u32;
                    if shift >= bits { !0u128 } else { (l as i128 >> shift) as u128 }
                } else {
                    l >> (r as u32)
                }
            }
            Eq => if l == r { 1 } else { 0 },
            Ne => if l != r { 1 } else { 0 },
            LtSigned => {
                let li = l as i128;
                let ri = r as i128;
                if li < ri { 1 } else { 0 }
            }
            LtUnsigned => if l < r { 1 } else { 0 },
            LeSigned => {
                let li = l as i128;
                let ri = r as i128;
                if li <= ri { 1 } else { 0 }
            }
            LeUnsigned => if l <= r { 1 } else { 0 },
            GtSigned => {
                let li = l as i128;
                let ri = r as i128;
                if li > ri { 1 } else { 0 }
            }
            GtUnsigned => if l > r { 1 } else { 0 },
            GeSigned => {
                let li = l as i128;
                let ri = r as i128;
                if li >= ri { 1 } else { 0 }
            }
            GeUnsigned => if l >= r { 1 } else { 0 },
            SQAdd | UQAdd => {
                // Saturating add - simplified
                l.saturating_add(r)
            }
            SQSub | UQSub => {
                // Saturating sub - simplified
                l.saturating_sub(r)
            }
        })
    }

    fn eval_unary_op(&self, op: &UnaryOp, v: u128) -> InterpreterResult<u128> {
        use UnaryOp::*;
        Ok(match op {
            Not => !v,
            Neg => v.wrapping_neg(),
            PopCnt => v.count_ones() as u128,
        })
    }

    fn eval_shift(&self, kind: &crate::primitives::ShiftKind, v: u128, a: u128) -> InterpreterResult<u128> {
        use crate::primitives::ShiftKind;
        let shift = a as u32;
        Ok(match kind {
            ShiftKind::Lsl => v << shift,
            ShiftKind::Lsr => v >> shift,
            ShiftKind::Asr => {
                let bits = 128; // TODO
                if v & (1u128 << (bits - 1)) != 0 {
                    if shift >= bits { !0u128 } else { (v as i128 >> shift) as u128 }
                } else {
                    v >> shift
                }
            }
            ShiftKind::Ror => {
                let bits = 128; // TODO
                let shift = shift % bits;
                if shift == 0 { v } else { (v >> shift) | (v << (bits - shift)) }
            }
        })
    }

    fn eval_extend(&self, kind: &crate::primitives::ExtendKind, v: u128, to: BitWidth) -> InterpreterResult<u128> {
        use crate::primitives::ExtendKind;
        let bits = to.bits();
        let src_bits = match kind {
            ExtendKind::Uxtb | ExtendKind::Sxtb => 8,
            ExtendKind::Uxth | ExtendKind::Sxth => 16,
            ExtendKind::Uxtw | ExtendKind::Sxtw => 32,
            ExtendKind::Uxtx | ExtendKind::Sxtx => 64,
        };
        let mask = if src_bits == 128 { !0u128 } else { (1u128 << src_bits) - 1 };
        let v = v & mask;
        let target_mask = if bits == 128 { !0u128 } else { (1u128 << bits) - 1 };
        if matches!(kind, ExtendKind::Sxtb | ExtendKind::Sxth | ExtendKind::Sxtw | ExtendKind::Sxtx) {
            // Sign extend
            let sign_bit = (v >> (src_bits - 1)) & 1;
            if sign_bit == 1 {
                Ok((v | (!0u128 << src_bits)) & target_mask)
            } else {
                Ok(v & target_mask)
            }
        } else {
            Ok(v & target_mask)
        }
    }

    fn eval_extend_reg(&self, kind: &crate::primitives::ExtendKind, reg_val: u128, shift: u128, to: crate::types::Width) -> InterpreterResult<u128> {
        // Shift the register value first
        let shifted = reg_val << shift;
        let bits = match to {
            crate::types::Width::Fixed(bw) => bw.bits(),
            _ => 64,
        };
        self.eval_extend(kind, shifted, BitWidth::new(bits).unwrap())
    }

    fn eval_call(&mut self, func: &str, args: &[Expr]) -> InterpreterResult<u128> {
        match func {
            "UInt" => {
                let v = self.eval_expr(&args[0])?;
                Ok(v)
            }
            "SInt" => {
                let v = self.eval_expr(&args[0])?;
                Ok(v)
            }
            "ZeroExtend" => {
                let v = self.eval_expr(&args[0])?;
                let to = args.get(1).and_then(|e| match e {
                    Expr::Const { value, ty } => Some(*ty),
                    _ => None,
                }).unwrap_or(BitWidth::W64);
                self.eval_expr(&Expr::ZeroExtend { value: Box::new(args[0].clone()), to })
            }
            "SignExtend" => {
                let v = self.eval_expr(&args[0])?;
                let to = args.get(1).and_then(|e| match e {
                    Expr::Const { value, ty } => Some(*ty),
                    _ => None,
                }).unwrap_or(BitWidth::W64);
                self.eval_expr(&Expr::SignExtend { value: Box::new(args[0].clone()), to })
            }
            "IsZero" => {
                let v = self.eval_expr(&args[0])?;
                Ok(if v == 0 { 1 } else { 0 })
            }
            "ConditionHolds" => {
                // Handled by Expr::ConditionHolds
                Ok(0)
            }
            "align_address" => {
                if args.len() >= 2 {
                    let addr = self.eval_expr(&args[0])?;
                    let align = self.eval_expr(&args[1])?;
                    Ok(crate::shared::bit_ops::align(addr as u64, align as u64) as u128)
                } else {
                    Ok(0)
                }
            }
            _ => Ok(0), // Unknown call - return 0
        }
    }

    fn eval_generic_prim(&mut self, name: &str, type_args: &[BitWidth], args: &[Expr]) -> InterpreterResult<u128> {
        match name {
            "UInt" | "SInt" => {
                if let Some(arg) = args.first() {
                    self.eval_expr(arg)
                } else { Ok(0) }
            }
            "ZeroExtend" | "SignExtend" => {
                if let Some(arg) = args.first() {
                    let v = self.eval_expr(arg)?;
                    let to = type_args.first().copied().unwrap_or(BitWidth::W64);
                    if name == "ZeroExtend" {
                        Ok(v & ((1u128 << to.bits()) - 1))
                    } else {
                        let bits = to.bits();
                        let sign_bit = (v >> (bits - 1)) & 1;
                        if sign_bit == 1 {
                            Ok(v | (!0u128 << bits))
                        } else {
                            Ok(v)
                        }
                    }
                } else { Ok(0) }
            }
            _ => Ok(0),
        }
    }

    fn match_pattern(&mut self, patterns: &[crate::stmt::MatchPattern], value: &u128) -> InterpreterResult<bool> {
        for pat in patterns {
            match pat {
                crate::stmt::MatchPattern::Constant(expr) => {
                    let pat_val = self.eval_expr(expr)?;
                    if *value == pat_val {
                        return Ok(true);
                    }
                }
                crate::stmt::MatchPattern::Wildcard => return Ok(true),
                crate::stmt::MatchPattern::Binding(_) => return Ok(true),
            }
        }
        Ok(false)
    }

    fn expr_to_bool(&self, val: &u128) -> InterpreterResult<bool> {
        Ok(*val != 0)
    }

    fn expr_to_u64(&self, val: &u128) -> InterpreterResult<u64> {
        Ok(*val as u64)
    }

    fn read_reg(&self, reg: &Reg) -> u128 {
        match reg {
            Reg::X(n) => {
                if *n < 31 {
                    self.state.gpr.x[*n as usize] as u128
                } else {
                    0
                }
            }
            Reg::Xzr => 0,
            Reg::W(n) => {
                if *n < 31 {
                    self.state.gpr.x[*n as usize] as u128 & 0xFFFF_FFFF
                } else {
                    0
                }
            }
            Reg::Wzr => 0,
            Reg::V(n) => {
                if *n < 32 {
                    self.state.simd.v[*n as usize] as u128
                } else {
                    0
                }
            }
        }
    }

    fn write_reg(&mut self, reg: &Reg, value: &u128) {
        match reg {
            Reg::X(n) => {
                if *n < 31 {
                    self.state.gpr.x[*n as usize] = *value as u64;
                }
            }
            Reg::Xzr => {}
            Reg::W(n) => {
                if *n < 31 {
                    self.state.gpr.x[*n as usize] = (self.state.gpr.x[*n as usize] & 0xFFFF_FFFF_0000_0000) | (*value as u64 & 0xFFFF_FFFF);
                }
            }
            Reg::Wzr => {}
            Reg::V(n) => {
                if *n < 32 {
                    self.state.simd.v[*n as usize] = *value as u128;
                }
            }
        }
    }

    fn write_sp(&mut self, value: &u128) {
        self.state.gpr.sp = *value as u64;
    }

    fn write_special(&mut self, reg: &SpecialReg, value: &u128) {
        match reg {
            SpecialReg::Pc64 | SpecialReg::Pc => {
                self.state.gpr.pc = *value as u64;
            }
            SpecialReg::Sp => {
                self.state.gpr.sp = *value as u64;
            }
            _ => {}
        }
    }

    fn read_special(&self, reg: &SpecialReg) -> u128 {
        match reg {
            SpecialReg::Pc64 | SpecialReg::Pc => self.state.gpr.pc as u128,
            SpecialReg::Sp => self.state.gpr.sp as u128,
            _ => 0,
        }
    }

    fn read_flag(&self, flag: &Flag) -> bool {
        match flag {
            Flag::N => self.state.pstate.flags.n,
            Flag::Z => self.state.pstate.flags.z,
            Flag::C => self.state.pstate.flags.c,
            Flag::V => self.state.pstate.flags.v,
        }
    }

    fn write_flag(&mut self, flag: &Flag, value: bool) {
        match flag {
            Flag::N => self.state.pstate.flags.n = value,
            Flag::Z => self.state.pstate.flags.z = value,
            Flag::C => self.state.pstate.flags.c = value,
            Flag::V => self.state.pstate.flags.v = value,
        }
    }

    /// Read memory at virtual address with specified size
    pub fn memory_read(&mut self, address: u128, size: MemSize) -> InterpreterResult<u128> {
        let vaddr = address as u64;
        let size_bytes = size.width().bits() / 8;

        // Check SP alignment if accessing via SP register
        if self.is_sp_relative_access() {
            let sp = self.state.gpr.sp as u64;
            check_sp_alignment(sp, self.state.pstate.current_el(), self.state.sys.sctlr_el1)
                .map_err(InterpreterError::MemoryFault)?;
        }

        // Create access descriptor for the read
        let accdesc = AccessDescriptor {
            acctype: AccessType::GPR,
            el: self.state.pstate.current_el(),
            ss: SecurityState::NonSecure,
            read: true,
            write: false,
            ..Default::default()
        };

        // Perform address translation
        let tcr_config = self.state.sys.tcr_el1_config;
        let ttbr0 = self.state.sys.ttbr0_el1;
        let ttbr1 = self.state.sys.ttbr1_el1;

        let addrdesc = aarch64_translate_address(
            &self.state.memory,
            vaddr,
            &accdesc,
            true, // aligned
            size_bytes as u64,
            &tcr_config,
            ttbr0,
            ttbr1,
        ).map_err(InterpreterError::MemoryFault)?;

        // Read from physical address
        let data = self.state.memory.read_phys(addrdesc.paddress.address, size_bytes as u64)
            .map_err(InterpreterError::MemoryFault)?;

        // Zero-extend to 128 bits
        Ok(data as u128)
    }

    /// Write memory at virtual address with specified size
    pub fn memory_write(&mut self, address: u128, value: u128, size: MemSize) -> InterpreterResult<()> {
        let vaddr = address as u64;
        let size_bytes = size.width().bits() / 8;

        // Check SP alignment if accessing via SP register
        if self.is_sp_relative_access() {
            let sp = self.state.gpr.sp as u64;
            check_sp_alignment(sp, self.state.pstate.current_el(), self.state.sys.sctlr_el1)
                .map_err(InterpreterError::MemoryFault)?;
        }

        // Create access descriptor for the write
        let accdesc = AccessDescriptor {
            acctype: AccessType::GPR,
            el: self.state.pstate.current_el(),
            ss: SecurityState::NonSecure,
            read: false,
            write: true,
            ..Default::default()
        };

        // Perform address translation
        let tcr_config = self.state.sys.tcr_el1_config;
        let ttbr0 = self.state.sys.ttbr0_el1;
        let ttbr1 = self.state.sys.ttbr1_el1;

        let addrdesc = aarch64_translate_address(
            &self.state.memory,
            vaddr,
            &accdesc,
            true, // aligned
            size_bytes as u64,
            &tcr_config,
            ttbr0,
            ttbr1,
        ).map_err(InterpreterError::MemoryFault)?;

        // Write to physical address
        self.state.memory.write_phys(addrdesc.paddress.address, size_bytes as u64, value as u64)
            .map_err(InterpreterError::MemoryFault)
    }

    /// Check if current memory access is SP-relative
    fn is_sp_relative_access(&self) -> bool {
        // Simplified: in a real implementation, this would track
        // whether the base register for the memory access is X31 (SP)
        // For now, we'll check if we're at EL0 and SCTLR has SP alignment checks enabled
        self.state.pstate.current_el() == ExceptionLevel::EL0 &&
        self.state.sys.sctlr_el1_config.sa
    }

    /// Evaluate atomic read-modify-write operation
    pub fn eval_atomic_rmw(&self, op: AtomicRMWOp, old_val: u128, new_val: u128, size: MemSize) -> InterpreterResult<u128> {
        let bits = size.width().bits();
        let mask = if bits == 128 { !0u128 } else { (1u128 << bits) - 1 };
        
        let old = old_val & mask;
        let new = new_val & mask;
        
        let result = match op {
            AtomicRMWOp::Add => old.wrapping_add(new),
            AtomicRMWOp::Bic => old & !new,
            AtomicRMWOp::Eor => old ^ new,
            AtomicRMWOp::Orr => old | new,
            AtomicRMWOp::Smax => {
                let old_s = old as i128;
                let new_s = new as i128;
                if old_s > new_s { old } else { new }
            }
            AtomicRMWOp::Smin => {
                let old_s = old as i128;
                let new_s = new as i128;
                if old_s < new_s { old } else { new }
            }
            AtomicRMWOp::Umax => if old > new { old } else { new },
            AtomicRMWOp::Umin => if old < new { old } else { new },
            AtomicRMWOp::Swp => new,
        };
        
        Ok(result & mask)
    }

    pub fn add_with_carry(&self, x: u128, y: u128, carry_in: bool, width: BitWidth) -> InterpreterResult<(u128, ConditionFlags)> {
        let bits = width.bits();
        let mask = if bits == 128 { !0u128 } else { (1u128 << bits) - 1 };
        let x_masked = x & mask;
        let y_masked = y & mask;
        let c = if carry_in { 1u128 } else { 0u128 };
        let unsigned_sum = x_masked + y_masked + c;
        let result = unsigned_sum & mask;

        // Signed interpretation
        let sx = if x_masked & (1u128 << (bits - 1)) != 0 {
            (x_masked | (!0u128 << bits)) as i128
        } else {
            x_masked as i128
        };
        let sy = if y_masked & (1u128 << (bits - 1)) != 0 {
            (y_masked | (!0u128 << bits)) as i128
        } else {
            y_masked as i128
        };
        let signed_sum = sx + sy + c as i128;

        let n = (result >> (bits - 1)) & 1 == 1;
        let z = result == 0;
        let c_flag = unsigned_sum > mask;
        let v = {
            let smax = match bits {
                8 => i8::MAX as i128,
                16 => i16::MAX as i128,
                32 => i32::MAX as i128,
                64 => i64::MAX as i128,
                128 => i128::MAX,
                _ => 0,
            };
            let smin = match bits {
                8 => i8::MIN as i128,
                16 => i16::MIN as i128,
                32 => i32::MIN as i128,
                64 => i64::MIN as i128,
                128 => i128::MIN,
                _ => 0,
            };
            signed_sum > smax || signed_sum < smin
        };

        let flags = ConditionFlags { n, z, c: c_flag, v };
        Ok((result, flags))
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BinOp, Expr, UnaryOp};
    use crate::primitives::Condition;
    use crate::stmt::Stmt;
    use crate::types::{BitWidth, Reg, SpecialReg, ValueType, Width};

    #[test]
    fn test_interpreter_add() {
        let mut interp = Interpreter::new();
        interp.state.gpr.write_x(0, 10);
        interp.state.gpr.write_x(1, 20);

        // result = X0 + X1
        let expr = Expr::Binary {
            op: BinOp::Add,
            left: Box::new(Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }),
            right: Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_interpreter_add_with_carry() {
        let mut interp = Interpreter::new();

        let (result, flags) = interp.add_with_carry(10, 20, false, BitWidth::W64).unwrap();
        assert_eq!(result, 30);
        assert!(!flags.n);
        assert!(!flags.z);
        assert!(!flags.c);
        assert!(!flags.v);

        // Test carry
        let (result, flags) = interp.add_with_carry(u64::MAX as u128, 1, false, BitWidth::W64).unwrap();
        assert_eq!(result, 0);
        assert!(!flags.n);
        assert!(flags.z);
        assert!(flags.c);
        assert!(!flags.v);

        // Test overflow
        let (result, flags) = interp.add_with_carry(0x7FFF_FFFF_FFFF_FFFFu128, 1, false, BitWidth::W64).unwrap();
        assert_eq!(result, 0x8000_0000_0000_0000);
        assert!(flags.n);
        assert!(!flags.z);
        assert!(!flags.c);
        assert!(flags.v);
    }

    #[test]
    fn test_interpreter_condition_holds() {
        let mut interp = Interpreter::new();
        interp.state.pstate.flags.z = true;

        let expr = Expr::ConditionHolds { condition: Condition::Eq };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 1);

        let expr = Expr::ConditionHolds { condition: Condition::Ne };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_interpreter_branch() {
        let mut interp = Interpreter::new();
        interp.state.gpr.write_pc(0x1000);

        let stmt = Stmt::Branch {
            target: Expr::Const { value: 0x2000, ty: BitWidth::W64 },
        };
        let result = interp.execute_stmt(&stmt);
        assert!(matches!(result, Err(InterpreterError::BranchTaken { target }) if target == 0x2000));
        assert_eq!(interp.state.gpr.read_pc(), 0x2000);
    }

    #[test]
    fn test_interpreter_branch_conditional() {
        let mut interp = Interpreter::new();
        interp.state.gpr.write_pc(0x1000);
        interp.state.pstate.flags.z = true;

        // B.EQ taken
        let stmt = Stmt::BranchConditional {
            condition: Expr::ConditionHolds { condition: Condition::Eq },
            target: Expr::Const { value: 0x2000, ty: BitWidth::W64 },
        };
        let result = interp.execute_stmt(&stmt);
        assert!(matches!(result, Err(InterpreterError::BranchTaken { target }) if target == 0x2000));
        assert_eq!(interp.state.gpr.read_pc(), 0x2000);
    }

    #[test]
    fn test_interpreter_write_reg() {
        let mut interp = Interpreter::new();

        let stmt = Stmt::WriteReg {
            reg: Reg::X(0),
            value: Expr::Const { value: 42, ty: BitWidth::W64 },
        };
        interp.execute_stmt(&stmt).unwrap();
        assert_eq!(interp.state.gpr.read_x(0), 42);
    }

    #[test]
    fn test_interpreter_write_flag() {
        let mut interp = Interpreter::new();

        let stmt = Stmt::WriteFlag {
            flag: Flag::Z,
            value: Expr::Bool(true),
        };
        interp.execute_stmt(&stmt).unwrap();
        assert!(interp.state.pstate.flags.z);
    }

    #[test]
    fn test_interpreter_select() {
        let mut interp = Interpreter::new();
        interp.state.pstate.flags.z = true;

        let expr = Expr::Select {
            condition: Box::new(Expr::ConditionHolds { condition: Condition::Eq }),
            when_true: Box::new(Expr::Const { value: 100, ty: BitWidth::W64 }),
            when_false: Box::new(Expr::Const { value: 200, ty: BitWidth::W64 }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_interpreter_zero_extend() {
        let mut interp = Interpreter::new();

        let expr = Expr::ZeroExtend {
            value: Box::new(Expr::Const { value: 0xFF, ty: BitWidth::W8 }),
            to: BitWidth::W64,
        };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 0xFF);
    }

    #[test]
    fn test_interpreter_sign_extend() {
        let mut interp = Interpreter::new();

        // Positive value
        let expr = Expr::SignExtend {
            value: Box::new(Expr::Const { value: 0x7F, ty: BitWidth::W8 }),
            to: BitWidth::W64,
        };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 0x7F);

        // Negative value (sign bit set in 8-bit)
        let expr = Expr::SignExtend {
            value: Box::new(Expr::Const { value: 0xFF, ty: BitWidth::W8 }),
            to: BitWidth::W64,
        };
        let result = interp.eval_expr(&expr).unwrap();
        assert_eq!(result, 0xFFFF_FFFF_FFFF_FFFFu128);
    }
}
