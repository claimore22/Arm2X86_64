use crate::expr::{BinOp, Expr, UnaryOp};
use crate::instruction::{DecodeExpr, DecodeSpec, DecodeStmt, Instruction};
use crate::primitives::{SVESpecialOp, SVEPredicateOp, SVEVectorReduceOp, SVEVectorTernaryOp, SVEVectorPairwiseOp, SVEVectorBinOp};
use crate::stmt::{MatchArm, MatchPattern, Stmt};
use crate::types::{BitWidth, Reg, ValueType, Width};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IrError {
    #[error("instruction has no mnemonic")]
    EmptyMnemonic,

    #[error("instruction semantic body is empty")]
    EmptySemantics,

    #[error("constant value {value} does not fit in {width}-bit value")]
    ConstantOutOfRange { value: u128, width: u16 },

    #[error("binary operation has incompatible operand types")]
    BinaryOperandTypeMismatch,

    #[error("binary operation result type is invalid")]
    BinaryResultTypeMismatch,

    #[error("unary operation has incompatible operand type")]
    UnaryOperandTypeMismatch,

    #[error("condition expression must have boolean type")]
    ConditionNotBoolean,

    #[error("select condition must be boolean")]
    SelectConditionNotBoolean,

    #[error("select branches must have matching types")]
    SelectBranchTypeMismatch,

    #[error("tuple element index {index} is invalid")]
    InvalidTupleElement { index: u8 },

    #[error("tuple element type is invalid")]
    InvalidTupleElementType,

    #[error("add-with-carry operands must have matching widths")]
    AddWithCarryWidthMismatch,

    #[error("add-with-carry carry input must be boolean")]
    AddWithCarryCarryNotBoolean,

    #[error("add-with-carry width must be 32 or 64 bits")]
    InvalidAddWithCarryWidth,

    #[error("zero extension must increase width")]
    InvalidZeroExtension,

    #[error("sign extension must increase width")]
    InvalidSignExtension,

    #[error("shift amount must be an integer bit-vector")]
    InvalidShiftAmount,

    #[error("indexed register index must be a 32-bit value")]
    InvalidRegisterIndex,

    #[error("write-SP width must be 32 or 64 bits")]
    InvalidSpWidth,

    #[error("empty decode expression")]
    EmptyDecodeExpression,

    #[error("empty decode identifier")]
    EmptyDecodeIdentifier,

    #[error("expression type could not be determined")]
    UnknownExpressionType,

    #[error("instruction provenance is missing")]
    MissingProvenance,

    #[error("bit index {index} is outside the expression width")]
    InvalidBitIndex { index: u16 },
}

pub fn validate_instruction(instruction: &Instruction) -> Result<(), IrError> {
    if instruction.id.mnemonic.is_empty() {
        return Err(IrError::EmptyMnemonic);
    }

    if instruction.provenance.source_path.is_empty()
        || instruction.provenance.source_sha256.is_empty()
        || instruction.provenance.clean_sha256.is_empty()
    {
        return Err(IrError::MissingProvenance);
    }

    if instruction.semantics.is_empty() {
        return Err(IrError::EmptySemantics);
    }

    if let Some(decode) = &instruction.decode {
        validate_decode(decode)?;
    }

    for statement in &instruction.semantics {
        validate_stmt(statement)?;
    }

    Ok(())
}

fn validate_decode(decode: &DecodeSpec) -> Result<(), IrError> {
    for statement in &decode.statements {
        match statement {
            DecodeStmt::Let { name, value } => {
                if name.is_empty() {
                    return Err(IrError::EmptyDecodeIdentifier);
                }

                validate_decode_expr(value)?;
            }

            DecodeStmt::Require { condition } => {
                validate_decode_expr(condition)?;
            }

            DecodeStmt::Undefined => {}
        }
    }

    Ok(())
}

fn validate_decode_expr(expr: &DecodeExpr) -> Result<(), IrError> {
    match expr {
        DecodeExpr::Identifier(name) => {
            if name.is_empty() {
                return Err(IrError::EmptyDecodeIdentifier);
            }
        }

        DecodeExpr::Immediate(_) => {}

        DecodeExpr::UInt(name) => {
            if name.is_empty() {
                return Err(IrError::EmptyDecodeIdentifier);
            }
        }

        DecodeExpr::In { value, values } => {
            validate_decode_expr(value)?;

            if values.is_empty() {
                return Err(IrError::EmptyDecodeExpression);
            }
        }

        DecodeExpr::Eq { left, right } => {
            validate_decode_expr(left)?;
            validate_decode_expr(right)?;
        }

        DecodeExpr::Not { value } => {
            validate_decode_expr(value)?;
        }

        DecodeExpr::ShiftLeft { value, .. } => {
            validate_decode_expr(value)?;
        }

        DecodeExpr::Add { left, right } => {
            validate_decode_expr(left)?;
            validate_decode_expr(right)?;
        }
    }

    Ok(())
}

fn validate_stmt(statement: &Stmt) -> Result<(), IrError> {
    match statement {
        Stmt::WriteReg { reg, value } => {
            validate_expr(value)?;

            let expected_width = match reg {
                Reg::X(_) => BitWidth::W64,
                Reg::W(_) => BitWidth::W32,
                Reg::V(_) => BitWidth::W128,
            };

            let value_type = expr_type(value);
            match reg {
                Reg::V(_) => {
                    // V registers can accept scalar values up to 128 bits
                    // (e.g., FMAXV writes 32/64-bit scalar to V register)
                    match value_type {
                        Some(ValueType::Bits(Width::Fixed(w))) if w.bits() <= 128 => {}
                        _ => return Err(IrError::BinaryOperandTypeMismatch),
                    }
                }
                _ => {
                    if value_type != Some(ValueType::Bits(Width::Fixed(expected_width))) {
                        return Err(IrError::BinaryOperandTypeMismatch);
                    }
                }
            }
        }

        Stmt::WriteIndexedReg {
            index,
            value,
            width,
        } => {
            validate_expr(index)?;

            if !is_u32_expr(index) {
                return Err(IrError::InvalidRegisterIndex);
            }

            validate_expr(value)?;
            validate_value_width(value, *width)?;
        }

        Stmt::WriteSP { value, width } => {
            validate_expr(value)?;

            match width {
                Width::Fixed(BitWidth::W32) | Width::Fixed(BitWidth::W64) => {}

                Width::Fixed(_) | Width::InstructionDataSize => {
                    return Err(IrError::InvalidSpWidth);
                }
            }

            validate_value_width(value, *width)?;
        }

        Stmt::WriteSpecial { value, width, .. } => {
            validate_expr(value)?;
            validate_value_width(value, *width)?;
        }

        Stmt::WritePredReg { value, pl, .. } => {
            validate_expr(value)?;
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*pl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::WriteScalableVecReg { value, vl, .. } => {
            validate_expr(value)?;
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::WriteFlag { value, .. } => {
            validate_expr(value)?;

            if !matches!(expr_type(value), Some(ValueType::Bool)) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::MemWrite { address, value, .. } => {
            validate_expr(address)?;
            validate_expr(value)?;

            if !is_bit_vector_expr(address) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::MemRead { address, dest: _, size: _, .. } => {
            validate_expr(address)?;

            if !is_bit_vector_expr(address) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::Let { name, value } => {
            if name.is_empty() {
                return Err(IrError::EmptyMnemonic);
            }

            validate_expr(value)?;
        }

        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            validate_expr(condition)?;

            if !matches!(expr_type(condition), Some(ValueType::Bool)) {
                return Err(IrError::ConditionNotBoolean);
            }

            for statement in then_body {
                validate_stmt(statement)?;
            }

            for statement in else_body {
                validate_stmt(statement)?;
            }
        }

        Stmt::Branch { target } => {
            validate_expr(target)?;

            let target_type = expr_type(target).ok_or(IrError::UnknownExpressionType)?;

            if !is_bit_vector_type(target_type) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::BranchConditional { condition, target } => {
            validate_expr(condition)?;

            if !matches!(expr_type(condition), Some(ValueType::Bool)) {
                return Err(IrError::ConditionNotBoolean);
            }

            validate_expr(target)?;

            if !is_bit_vector_expr(target) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Stmt::RaiseException(_) => {}

        Stmt::Discard { value } => {
            validate_expr(value)?;
        }

        Stmt::Assign { name: _, value } => {
            validate_expr(value)?;
        }

        Stmt::AssignPlace { place, value } => {
            validate_expr(place)?;
            validate_expr(value)?;

            // Validate that place is a valid place expression:
            // - Slice (bit slice of a register/memory)
            // - FieldAccess (architectural field like PSTATE.Z)
            // - Name via ReadReg (register assignment)
            match place {
                Expr::Slice { .. } | Expr::FieldAccess { .. } | Expr::ReadReg { .. } => {}
                _ => return Err(IrError::BinaryOperandTypeMismatch),
            }
        }

        Stmt::Match { scrutinee, arms } => {
            validate_expr(scrutinee)?;
            for arm in arms {
                for pattern in &arm.patterns {
                    validate_match_pattern(pattern)?;
                }
                if let Some(guard) = &arm.guard {
                    validate_expr(guard)?;
                }
                for stmt in &arm.body {
                    validate_stmt(stmt)?;
                }
            }
        }

        Stmt::While { condition, body, max_iterations: _ } => {
            validate_expr(condition)?;
            if !matches!(expr_type(condition), Some(ValueType::Bool)) {
                return Err(IrError::ConditionNotBoolean);
            }
            for stmt in body {
                validate_stmt(stmt)?;
            }
        }

        Stmt::Return { value } => {
            if let Some(v) = value {
                validate_expr(v)?;
            }
        }
    }

    Ok(())
}

fn validate_match_pattern(pattern: &MatchPattern) -> Result<(), IrError> {
    match pattern {
        MatchPattern::Constant(expr) => validate_expr(expr),
        MatchPattern::Wildcard => Ok(()),
        MatchPattern::Binding(_) => Ok(()),
    }
}

fn validate_expr(expr: &Expr) -> Result<(), IrError> {
    match expr {
        Expr::Const { value, ty } => {
            validate_constant(*value, *ty)?;
        }

        Expr::Bool(_) => {}

        Expr::ReadReg { .. } => {}

        Expr::ReadIndexedReg { index, .. } => {
            validate_expr(index)?;

            if !is_u32_expr(index) {
                return Err(IrError::InvalidRegisterIndex);
            }
        }

        Expr::ReadSP { width } => {
            if !matches!(*width, BitWidth::W32 | BitWidth::W64) {
                return Err(IrError::InvalidSpWidth);
            }
        }

        Expr::ReadSpecial { .. } => {}

        Expr::SystemReg { .. } => {}

        Expr::ReadFlag { .. } => {}

        Expr::Binary {
            op,
            left,
            right,
            ty,
        } => {
            validate_expr(left)?;
            validate_expr(right)?;
            validate_binary(*op, left, right, *ty)?;
        }

        Expr::Unary { op, value, ty } => {
            validate_expr(value)?;
            validate_unary(*op, value, *ty)?;
        }

        Expr::ScalarFPBinary { op: _, left, right, ty } => {
            validate_expr(left)?;
            validate_expr(right)?;

            // Both operands must be bit vectors of the specified element size
            let expected = ValueType::Bits(Width::Fixed(*ty));
            if expr_type(left) != Some(expected) || expr_type(right) != Some(expected) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Element size must be valid FP size (16, 32, or 64 bits)
            if !matches!(*ty, BitWidth::W16 | BitWidth::W32 | BitWidth::W64) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Shift {
            value, amount, ty, ..
        } => {
            validate_expr(value)?;
            validate_expr(amount)?;

            if !is_bit_vector_expr(amount) {
                return Err(IrError::InvalidShiftAmount);
            }

            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*ty))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Extend { value, to, .. } => {
            validate_expr(value)?;

            let Some(ValueType::Bits(width)) = expr_type(value) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let Width::Fixed(from) = width else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if to.bits() <= from.bits() {
                return Err(IrError::InvalidZeroExtension);
            }
        }

        Expr::ExtendReg {
            register,
            shift,
            to: _,
            ..
        } => {
            validate_expr(register)?;
            validate_expr(shift)?;

            if !is_u32_expr(register) {
                return Err(IrError::InvalidRegisterIndex);
            }

            if !is_bit_vector_expr(shift) {
                return Err(IrError::InvalidShiftAmount);
            }
        }

        Expr::ZeroExtend { value, to } | Expr::SignExtend { value, to } => {
            validate_expr(value)?;

            let Some(ValueType::Bits(width)) = expr_type(value) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let Width::Fixed(from) = width else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if to.bits() < from.bits() {
                return Err(if matches!(expr, Expr::ZeroExtend { .. }) {
                    IrError::InvalidZeroExtension
                } else {
                    IrError::InvalidSignExtension
                });
            }
        }

        Expr::BitTest { value, bit } => {
            validate_expr(value)?;

            let Some(ValueType::Bits(Width::Fixed(width))) = expr_type(value) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if *bit >= width.bits() {
                return Err(IrError::InvalidBitIndex { index: *bit });
            }
        }

        Expr::Extract { value, lsb, width } => {
            validate_expr(value)?;

            let Some(ValueType::Bits(value_width)) = expr_type(value) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let Width::Fixed(value_width) = value_width else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let end = u32::from(*lsb) + u32::from(width.bits());

            if end > u32::from(value_width.bits()) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Concat { high, low, width } => {
            validate_expr(high)?;
            validate_expr(low)?;

            let Some(ValueType::Bits(high_width)) = expr_type(high) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let Some(ValueType::Bits(low_width)) = expr_type(low) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            let (Width::Fixed(high_width), Width::Fixed(low_width)) = (high_width, low_width)
            else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if u32::from(high_width.bits()) + u32::from(low_width.bits()) != u32::from(width.bits())
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Replicate {
            value,
            count,
            element_width,
        } => {
            validate_expr(value)?;

            let expected = u32::from(*count) * u32::from(element_width.bits());

            let Some(ValueType::Bits(Width::Fixed(actual))) = expr_type(value) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if u32::from(actual.bits()) != u32::from(element_width.bits()) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expected == 0 || expected > 128 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorMap {
            op: _,
            operand,
            datasize,
            esize,
        } => {
            validate_expr(operand)?;

            let Some(ValueType::Bits(Width::Fixed(operand_datasize))) = expr_type(operand) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if operand_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorMap2 {
            op: _,
            left,
            right,
            datasize,
            esize,
        } => {
            validate_expr(left)?;
            validate_expr(right)?;

            let Some(ValueType::Bits(Width::Fixed(left_datasize))) = expr_type(left) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };
            let Some(ValueType::Bits(Width::Fixed(right_datasize))) = expr_type(right) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if left_datasize != *datasize || right_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorReduce {
            op: _,
            operand,
            datasize,
            esize,
        } => {
            validate_expr(operand)?;

            let Some(ValueType::Bits(Width::Fixed(operand_datasize))) = expr_type(operand) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if operand_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorPairwise {
            op: _,
            operand,
            operand_datasize,
            result_datasize,
            esize,
        } => {
            validate_expr(operand)?;

            let Some(ValueType::Bits(Width::Fixed(operand_actual_datasize))) = expr_type(operand) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if operand_actual_datasize != *operand_datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if operand_datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Result datasize must be half of operand datasize (for vector ADDP) or equal (for scalar ADDP)
            if result_datasize.bits() != operand_datasize.bits() / 2 && result_datasize.bits() != operand_datasize.bits() {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Pairwise requires even number of elements in operand
            let elements = operand_datasize.bits() / esize.bits();
            if elements % 2 != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorNarrowing {
            op: _,
            left,
            right,
            datasize,
            esize,
            round: _,
            part,
        } => {
            validate_expr(left)?;
            validate_expr(right)?;

            let Some(ValueType::Bits(Width::Fixed(left_datasize))) = expr_type(left) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };
            let Some(ValueType::Bits(Width::Fixed(right_datasize))) = expr_type(right) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if left_datasize != *datasize || right_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Source element size must be valid (8, 16, 32, 64, 128)
            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Part must be 0 or 1
            if *part > 1 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorPerm {
            op: _,
            operands,
            datasize,
            esize,
            imm: _,
        } => {
            if operands.is_empty() || operands.len() > 4 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            for operand in operands {
                validate_expr(operand)?;

                let Some(ValueType::Bits(Width::Fixed(operand_datasize))) = expr_type(operand) else {
                    return Err(IrError::BinaryOperandTypeMismatch);
                };

                if operand_datasize != *datasize {
                    return Err(IrError::BinaryOperandTypeMismatch);
                }
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorDeinterleave {
            op: _,
            operand,
            datasize,
            esize,
            scale: _,
            f8_format: _,
            fpcr: _,
            fpmr: _,
        } => {
            validate_expr(operand)?;

            let Some(ValueType::Bits(Width::Fixed(operand_datasize))) = expr_type(operand) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if operand_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // esize must be 8 for FP8
            if !matches!(*esize, BitWidth::W8) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorInterleave {
            op: _,
            left,
            right,
            datasize,
            esize,
            result_esize,
        } => {
            validate_expr(left)?;
            validate_expr(right)?;

            let Some(ValueType::Bits(Width::Fixed(left_datasize))) = expr_type(left) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };
            let Some(ValueType::Bits(Width::Fixed(right_datasize))) = expr_type(right) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if left_datasize != *datasize || right_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
            
            // Validate that the result datasize makes sense
            // For interleaving two vectors of elements at esize, the result has 
            // 2 * elements * result_esize bits = datasize * (result_esize / esize)
            let src_elements = datasize.bits() / esize.bits();
            let expected_result_bits = src_elements * 2 * result_esize.bits();
            if expected_result_bits != datasize.bits() {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::VectorDot {
            op,
            a,
            b,
            addend,
            datasize,
            esize,
            fpcr: _,
            fpmr: _,
            broadcast_index: _,
        } => {
            validate_expr(a)?;
            validate_expr(b)?;
            validate_expr(addend)?;

            let Some(ValueType::Bits(Width::Fixed(a_datasize))) = expr_type(a) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };
            let Some(ValueType::Bits(Width::Fixed(b_datasize))) = expr_type(b) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };
            let Some(ValueType::Bits(Width::Fixed(addend_datasize))) = expr_type(addend) else {
                return Err(IrError::BinaryOperandTypeMismatch);
            };

            if a_datasize != *datasize || b_datasize != *datasize || addend_datasize != *datasize {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // For BFDOT: source elements are 16-bit BF16, result is 32-bit FP32
            // For FP8Dot: source elements are 8-bit FP8, result is 16-bit FP16 (2-way) or 32-bit FP32 (4-way)
            // esize is the result element size
            if !matches!(*esize, BitWidth::W16 | BitWidth::W32) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if datasize.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Condition(_) => {}

        Expr::ConditionHolds { .. } => {}

        Expr::If { cond, then_branch, else_branch } => {
            validate_expr(cond)?;
            validate_expr(then_branch)?;
            validate_expr(else_branch)?;
        }

        Expr::Call { func: _, args } => {
            for arg in args {
                validate_expr(arg)?;
            }
        }

        Expr::FieldAccess { base, field } => {
            validate_expr(base)?;
            // If base is PSTATE, validate the field is a known PSTATE field
            if let Expr::FieldAccess { field: base_field, .. } = &**base {
                if base_field == "PSTATE" {
                    // Validate PSTATE field
                    match field.as_str() {
                        "N" | "Z" | "C" | "V" | "EL" | "SP" | "n" | "z" | "c" | "v" => {
                            // Valid PSTATE fields
                        }
                        _ => {
                            // Unknown PSTATE field - report but don't fail validation
                            // This allows for forward compatibility
                        }
                    }
                }
            }
        }

        Expr::EnumLit { .. } => {}

        Expr::Slice { value, lsb, msb } => {
            validate_expr(value)?;

            // Validate slice bounds: lsb <= msb
            if *lsb > *msb {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Validate bounds against value width
            if let Some(ValueType::Bits(Width::Fixed(width))) = expr_type(value) {
                let max_bit = width.bits().saturating_sub(1) as u16;
                if *msb > max_bit {
                    return Err(IrError::InvalidBitIndex { index: *msb });
                }
            }
        }

        Expr::GenericPrim { name: _, type_args: _, args } => {
            for arg in args {
                validate_expr(arg)?;
            }
        }

        Expr::PointerAddCheck { value, base } => {
            validate_expr(value)?;
            validate_expr(base)?;

            let want = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));

            if expr_type(value) != want || expr_type(base) != want {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

Expr::SysOp {
            op0,
            op1,
            crn,
            crm,
            op2,
            address,
        }
        | Expr::SysLOp {
            op0,
            op1,
            crn,
            crm,
            op2,
            address,
        } => {
            validate_expr(op0)?;
            validate_expr(op1)?;
            validate_expr(crn)?;
            validate_expr(crm)?;
            validate_expr(op2)?;
            validate_expr(address)?;

            let field = |bits: u16| Some(ValueType::Bits(Width::Fixed(BitWidth::new(bits)?)));
            let want = |bits: u16| field(bits).unwrap_or(ValueType::Bool);

            if expr_type(op0) != Some(want(2))
                || expr_type(op1) != Some(want(3))
                || expr_type(crn) != Some(want(4))
                || expr_type(crm) != Some(want(4))
                || expr_type(op2) != Some(want(3))
                || expr_type(address) != Some(ValueType::Bits(Width::Fixed(BitWidth::W64)))
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AuthDA { address, modifier, auth_combined }
        | Expr::AuthIA { address, modifier, auth_combined }
        | Expr::AuthIB { address, modifier, auth_combined }
        | Expr::AuthDB { address, modifier, auth_combined } => {
            validate_expr(address)?;
            validate_expr(modifier)?;
            validate_expr(auth_combined)?;

            let want64 = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));
            let want_bool = Some(ValueType::Bool);

            if expr_type(address) != want64
                || expr_type(modifier) != want64
                || expr_type(auth_combined) != want_bool
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AddPAC {
            address, modifier, ..
        } => {
            validate_expr(address)?;
            validate_expr(modifier)?;

            let want64 = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));
            if expr_type(address) != want64 || expr_type(modifier) != want64 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AddPAC2 {
            address,
            modifier1,
            modifier2,
            ..
        } => {
            validate_expr(address)?;
            validate_expr(modifier1)?;
            validate_expr(modifier2)?;

            let want64 = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));
            if expr_type(address) != want64
                || expr_type(modifier1) != want64
                || expr_type(modifier2) != want64
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AuthIA2 {
            address,
            modifier1,
            modifier2,
            auth_combined,
        } => {
            validate_expr(address)?;
            validate_expr(modifier1)?;
            validate_expr(modifier2)?;
            validate_expr(auth_combined)?;

            let want64 = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));
            let want_bool = Some(ValueType::Bool);

            if expr_type(address) != want64
                || expr_type(modifier1) != want64
                || expr_type(modifier2) != want64
                || expr_type(auth_combined) != want_bool
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AuthIB2 {
            address,
            modifier1,
            modifier2,
            auth_combined,
        } => {
            validate_expr(address)?;
            validate_expr(modifier1)?;
            validate_expr(modifier2)?;
            validate_expr(auth_combined)?;

            let want64 = Some(ValueType::Bits(Width::Fixed(BitWidth::W64)));
            let want_bool = Some(ValueType::Bool);

            if expr_type(address) != want64
                || expr_type(modifier1) != want64
                || expr_type(modifier2) != want64
                || expr_type(auth_combined) != want_bool
            {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ChooseTag {
            start_tag,
            tag_offset,
            gcr,
        } => {
            validate_expr(start_tag)?;
            validate_expr(tag_offset)?;
            validate_expr(gcr)?;

            if expr_type(start_tag) != Some(ValueType::Bits(Width::Fixed(BitWidth::W4))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(tag_offset) != Some(ValueType::Bits(Width::Fixed(BitWidth::W4))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(gcr) != Some(ValueType::Bits(Width::Fixed(BitWidth::W64))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::AddWithCarry {
            left,
            right,
            carry,
            width,
        } => {
            validate_expr(left)?;
            validate_expr(right)?;
            validate_expr(carry)?;

            if !matches!(expr_type(carry), Some(ValueType::Bool)) {
                return Err(IrError::AddWithCarryCarryNotBoolean);
            }

            if !matches!(*width, BitWidth::W32 | BitWidth::W64) {
                return Err(IrError::InvalidAddWithCarryWidth);
            }

            let expected = ValueType::Bits(Width::Fixed(*width));

            if expr_type(left) != Some(expected) || expr_type(right) != Some(expected) {
                return Err(IrError::AddWithCarryWidthMismatch);
            }
        }

        Expr::MemRead { address, .. } => {
            validate_expr(address)?;
        }

        Expr::MemAtomic {
            address,
            expected,
            new_value,
            size,
            ..
        } => {
            validate_expr(address)?;
            validate_expr(expected)?;
            validate_expr(new_value)?;

            if !is_bit_vector_expr(address) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            let expected_ty = ValueType::Bits(Width::Fixed(size.width()));
            if expr_type(expected) != Some(expected_ty) || expr_type(new_value) != Some(expected_ty) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::MemAtomicRMW {
            address,
            value,
            size,
            ..
        } => {
            validate_expr(address)?;
            validate_expr(value)?;

            if !is_bit_vector_expr(address) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            let value_ty = ValueType::Bits(Width::Fixed(size.width()));
            if expr_type(value) != Some(value_ty) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Select {
            condition,
            when_true,
            when_false,
            ty,
        } => {
            validate_expr(condition)?;
            validate_expr(when_true)?;
            validate_expr(when_false)?;

            if !matches!(expr_type(condition), Some(ValueType::Bool)) {
                return Err(IrError::SelectConditionNotBoolean);
            }

            if expr_type(when_true) != Some(*ty) || expr_type(when_false) != Some(*ty) {
                return Err(IrError::SelectBranchTypeMismatch);
            }
        }

        Expr::TupleElement { value, index, ty } => {
            validate_expr(value)?;

            let Expr::AddWithCarry { width, .. } = value.as_ref() else {
                return Err(IrError::InvalidTupleElement { index: *index });
            };

            let expected = match *index {
                // Stanford:
                // (result, nzcv) = AddWithCarry(...)
                0 => ValueType::Bits(Width::Fixed(*width)),
                1 => ValueType::Bits(Width::Fixed(BitWidth::W4)),
                _ => {
                    return Err(IrError::InvalidTupleElement { index: *index });
                }
            };

            if *ty != expected {
                return Err(IrError::InvalidTupleElementType);
            }
        }

        Expr::ChkFeat { value } => {
            validate_expr(value)?;

            // ChkFeat takes a 64-bit value and returns a 64-bit value
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(BitWidth::W64))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ChooseTagOrZero { exclude } => {
            validate_expr(exclude)?;

            // exclude set is provided as the full 64-bit source register
            if expr_type(exclude) != Some(ValueType::Bits(Width::Fixed(BitWidth::W64))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::MemTagRead { address } => {
            validate_expr(address)?;

            if expr_type(address) != Some(ValueType::Bits(Width::Fixed(BitWidth::W64))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::Crc32 {
            acc,
            value,
            data_width,
            ..
        } => {
            validate_expr(acc)?;
            validate_expr(value)?;

            // acc is always 32-bit
            if expr_type(acc) != Some(ValueType::Bits(Width::Fixed(BitWidth::W32))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // value width must match the declared data_width (8/16/32/64)
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*data_width))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ReadPredReg { pl, .. } => {
            // Predicate registers are PL bits (VL/8)
            if !matches!(*pl, BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128 | BitWidth::W256) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ReadScalableVecReg { vl, .. } => {
            // Scalable vector registers are VL bits
            if !matches!(*vl, BitWidth::W128 | BitWidth::W256 | BitWidth::W512 | BitWidth::W1024 | BitWidth::W2048) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::WritePredReg { value, pl, .. } => {
            validate_expr(value)?;
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*pl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::WriteScalableVecReg { value, vl, .. } => {
            validate_expr(value)?;
            if expr_type(value) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::PredicateMap { operand, pl, .. } => {
            validate_expr(operand)?;
            if expr_type(operand) != Some(ValueType::Bits(Width::Fixed(*pl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ScalableVectorMap {
            predicate,
            operand,
            destination,
            vl,
            esize,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(operand)?;
            validate_expr(destination)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(operand) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(destination) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ScalableVectorMap2 {
            op,
            predicate,
            left,
            right,
            destination,
            vl,
            esize,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(left)?;
            validate_expr(right)?;
            validate_expr(destination)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(left) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // For shift operations with immediate (like Sqshlu), right operand can be a constant
            // For vector-vector operations, right operand must be a vector (VL bits)
            let right_is_vector = expr_type(right) == Some(ValueType::Bits(Width::Fixed(*vl)));
            let right_is_constant = matches!(expr_type(right), Some(ValueType::Bits(Width::Fixed(w))) if w.bits() <= 64);
            
            // Sqshlu takes an immediate shift amount as right operand
            let is_sqshlu = matches!(*op, SVEVectorBinOp::Sqshlu);
            
            if !is_sqshlu && !right_is_vector {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
            
            if is_sqshlu && !right_is_constant {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(destination) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ScalableVectorMap3 {
            op,
            predicate,
            left,
            middle,
            right,
            destination,
            vl,
            esize,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(left)?;
            validate_expr(middle)?;
            validate_expr(right)?;
            validate_expr(destination)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(left) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // For shift operations with immediate (like Sshllb, Sshllt, Ushllb, Ushllt, Uqshrnb, Uqshrnt, Ursh), middle operand can be a constant
            let middle_is_vector = expr_type(middle) == Some(ValueType::Bits(Width::Fixed(*vl)));
            let middle_is_constant = matches!(expr_type(middle), Some(ValueType::Bits(Width::Fixed(w))) if w.bits() <= 64);
            
            // Sshllb, Sshllt, Ushllb, Ushllt, Uqshrnb, Uqshrnt, Ursh take an immediate shift amount as middle operand
            let is_sshll = matches!(*op, SVEVectorTernaryOp::Sshllb | SVEVectorTernaryOp::Sshllt | SVEVectorTernaryOp::Ushllb | SVEVectorTernaryOp::Ushllt | SVEVectorTernaryOp::Uqshrnb | SVEVectorTernaryOp::Uqshrnt | SVEVectorTernaryOp::Ursh);
            
            if !is_sshll && !middle_is_vector {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
            
            if is_sshll && !middle_is_constant {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(right) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(destination) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::PredicateReduce { operand, pl, .. } => {
            validate_expr(operand)?;
            if expr_type(operand) != Some(ValueType::Bits(Width::Fixed(*pl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ScalableVectorReduce {
            predicate,
            operand,
            vl,
            esize,
            initial_value,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(operand)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(operand) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Validate optional initial_value (for FADDA)
            if let Some(init) = initial_value {
                validate_expr(init)?;
                // Initial value should be a scalar of size esize
                if expr_type(init) != Some(ValueType::Bits(Width::Fixed(*esize))) {
                    return Err(IrError::BinaryOperandTypeMismatch);
                }
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::ScalableVectorPairwise {
            predicate,
            left,
            right,
            destination,
            vl,
            esize,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(left)?;
            validate_expr(right)?;
            validate_expr(destination)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(left) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(right) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(destination) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::SVEGatherScatter {
            predicate,
            base,
            index,
            destination,
            vl,
            esize,
            msize,
            ..
        } => {
            validate_expr(predicate)?;
            validate_expr(base)?;
            validate_expr(index)?;
            validate_expr(destination)?;

            let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(vl.bits() / 8).unwrap()));
            if expr_type(predicate) != Some(expected_pred) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            let base_type = expr_type(base);
            // For gather loads with vector base (vector + immediate index), base is vector (VL bits)
            // For gather loads with vector index (scalar + vector), base is scalar 64-bit
            // For contiguous loads, base is scalar 64-bit
            let is_vector_base = matches!(base_type, Some(ValueType::Bits(Width::Fixed(w))) if w == *vl);
            let is_scalar_base = matches!(base_type, Some(ValueType::Bits(Width::Fixed(BitWidth::W64))));
            if !(is_vector_base || is_scalar_base) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            // Index can be either a vector (vl bits) for gather with vector index,
            // or a 32-bit immediate for gather with immediate index (using 32-bit encoding),
            // or a 64-bit register (X register) for contiguous loads with scalar index,
            // or a 4-bit immediate for contiguous loads with immediate index (LD1B etc.),
            // or a 5-bit immediate for gather loads with immediate index (LD1B/H/W/D etc.),
            // or a 6-bit immediate for replicate loads with immediate index (LD1R etc.)
            let index_type = expr_type(index);
            if index_type != Some(ValueType::Bits(Width::Fixed(*vl))) &&
               index_type != Some(ValueType::Bits(Width::Fixed(BitWidth::W32))) &&
               index_type != Some(ValueType::Bits(Width::Fixed(BitWidth::W64))) &&
               index_type != Some(ValueType::Bits(Width::Fixed(BitWidth::W4))) &&
               index_type != Some(ValueType::Bits(Width::Fixed(BitWidth::new(5).unwrap()))) &&
               index_type != Some(ValueType::Bits(Width::Fixed(BitWidth::new(6).unwrap()))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if expr_type(destination) != Some(ValueType::Bits(Width::Fixed(*vl))) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*esize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if !matches!(*msize, BitWidth::W8 | BitWidth::W16 | BitWidth::W32 | BitWidth::W64 | BitWidth::W128) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            if vl.bits() % esize.bits() != 0 {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        Expr::SVESpecial {
            op,
            predicate,
            operands,
            vl,
            ..
        } => {
            if let Some(pred) = predicate {
                validate_expr(pred)?;
                // For CounterToPredicate, the predicate is a mask from CounterToPredicate (PL*4 bits)
                // For CountPred, it can be either a regular predicate (PL bits) or expanded mask (PL*4 bits)
                // For other operations, the predicate has PL bits (VL/8)
                let expected_pred_bits = if matches!(op, SVESpecialOp::CounterToPredicate) {
                    vl.bits() / 8 * 4  // PL * 4
                } else if matches!(op, SVESpecialOp::CountPred) {
                    // CountPred can accept both PL bits (regular predicate) or PL*4 bits (expanded mask)
                    // Check the actual predicate type and accept either
                    let pred_type = expr_type(pred);
                    let pl_bits = vl.bits() / 8;
                    let pl4_bits = pl_bits * 4;
                    let pred_bits = pred_type.and_then(|vt| match vt {
                        ValueType::Bits(Width::Fixed(bw)) => Some(bw.bits()),
                        _ => None,
                    }).unwrap_or(pl_bits);
                    if pred_bits == pl4_bits {
                        pl4_bits
                    } else {
                        pl_bits
                    }
                } else {
                    vl.bits() / 8       // PL
                };
                let expected_pred = ValueType::Bits(Width::Fixed(crate::types::BitWidth::new(expected_pred_bits as u16).unwrap()));
                if expr_type(pred) != Some(expected_pred) {
                    return Err(IrError::BinaryOperandTypeMismatch);
                }
            }
            for operand in operands {
                validate_expr(operand)?;
            }
            if !matches!(*vl, BitWidth::W128 | BitWidth::W256 | BitWidth::W512 | BitWidth::W1024 | BitWidth::W2048) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
            // Additional validation for Match/Nmatch operations
            if matches!(op, SVESpecialOp::Match | SVESpecialOp::Nmatch) {
                // Match/Nmatch require exactly 2 vector operands
                if operands.len() != 2 {
                    return Err(IrError::BinaryOperandTypeMismatch);
                }
                // Both operands must be scalable vector registers (VL bits)
                for operand in operands {
                    let operand_type = expr_type(operand);
                    if operand_type != Some(ValueType::Bits(Width::Fixed(*vl))) {
                        return Err(IrError::BinaryOperandTypeMismatch);
                    }
                }
            }
        }

        Expr::PredicateMap2 {
            left,
            right,
            pl,
            ..
        } => {
            validate_expr(left)?;
            validate_expr(right)?;

            // For predicate-to-predicate ops (And, Or, etc.), operands are predicates (PL bits)
            // For vector-to-predicate ops (FCMEQ, etc.), operands are vectors (VL bits)
            // Accept either predicates (PL bits) or vectors (VL bits: 128, 256, 512, 1024, 2048)
let left_type = expr_type(left);
            let right_type = expr_type(right);

            // Check if left_type is a predicate (PL bits) or vector (VL bits)
            let is_predicate = matches!(left_type, Some(ValueType::Bits(Width::Fixed(w))) if w == *pl);
            let is_vector = matches!(left_type,
                Some(ValueType::Bits(Width::Fixed(BitWidth::W128))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W256))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W512))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W1024))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W2048)))
            );

            if !(is_predicate || is_vector) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }

            let right_is_predicate = matches!(right_type, Some(ValueType::Bits(Width::Fixed(w))) if w == *pl);
            let right_is_vector = matches!(right_type,
                Some(ValueType::Bits(Width::Fixed(BitWidth::W128))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W256))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W512))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W1024))) |
                Some(ValueType::Bits(Width::Fixed(BitWidth::W2048)))
            );

            if !(right_is_predicate || right_is_vector) {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }
    }

    Ok(())
}

fn validate_binary(op: BinOp, left: &Expr, right: &Expr, result: ValueType) -> Result<(), IrError> {
    let left_type = expr_type(left);
    let right_type = expr_type(right);

    match op {
        BinOp::Eq | BinOp::Ne => {
            if left_type.is_none() || left_type != right_type || result != ValueType::Bool {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        BinOp::LtSigned
        | BinOp::LtUnsigned
        | BinOp::LeSigned
        | BinOp::LeUnsigned
        | BinOp::GtSigned
        | BinOp::GtUnsigned
        | BinOp::GeSigned
        | BinOp::GeUnsigned => {
            if !is_bit_vector_expr(left) || left_type != right_type || result != ValueType::Bool {
                return Err(IrError::BinaryOperandTypeMismatch);
            }
        }

        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Shl
        | BinOp::Lsr
        | BinOp::Asr
        | BinOp::SQAdd
        | BinOp::UQAdd
        | BinOp::SQSub
        | BinOp::UQSub => {
            if !is_bit_vector_expr(left)
                || left_type != right_type
                || result != left_type.unwrap_or(ValueType::Bool)
            {
                return Err(IrError::BinaryResultTypeMismatch);
            }
        }

        BinOp::And | BinOp::Or | BinOp::Xor => {
            match (left_type, right_type) {
                (Some(ValueType::Bool), Some(ValueType::Bool)) => {
                    if result != ValueType::Bool {
                        return Err(IrError::BinaryResultTypeMismatch);
                    }
                }
                (Some(ValueType::Bits(a)), Some(ValueType::Bits(b))) => {
                    if a != b || result != ValueType::Bits(a) {
                        return Err(IrError::BinaryResultTypeMismatch);
                    }
                }
                _ => return Err(IrError::BinaryOperandTypeMismatch),
            }
        }
    }

    Ok(())
}

fn validate_unary(op: UnaryOp, value: &Expr, result: ValueType) -> Result<(), IrError> {
    match op {
        UnaryOp::Not | UnaryOp::Neg | UnaryOp::PopCnt => {
            if expr_type(value) != Some(result) {
                return Err(IrError::UnaryOperandTypeMismatch);
            }
        }
    }

    Ok(())
}

fn validate_constant(value: u128, width: BitWidth) -> Result<(), IrError> {
    let bits = width.bits();

    if bits < 128 && value >= (1u128 << bits) {
        return Err(IrError::ConstantOutOfRange { value, width: bits });
    }

    Ok(())
}

fn validate_value_width(value: &Expr, expected: Width) -> Result<(), IrError> {
    let actual = expr_type(value);

    if actual == Some(ValueType::Bits(expected)) {
        Ok(())
    } else {
        Err(IrError::BinaryOperandTypeMismatch)
    }
}

fn is_u32_expr(expr: &Expr) -> bool {
    matches!(
        expr_type(expr),
        Some(ValueType::Bits(Width::Fixed(BitWidth::W32)))
    )
}

fn is_bit_vector_type(ty: ValueType) -> bool {
    matches!(ty, ValueType::Bits(_))
}

fn is_bit_vector_expr(expr: &Expr) -> bool {
    matches!(expr_type(expr), Some(ValueType::Bits(_)))
}

fn expr_type(expr: &Expr) -> Option<ValueType> {
    match expr {
        Expr::Const { ty, .. } => Some(ValueType::Bits(Width::Fixed(*ty))),

        Expr::Bool(_) => Some(ValueType::Bool),

        Expr::ReadReg { width, .. }
        | Expr::ReadIndexedReg { width, .. }
        | Expr::ReadSP { width, .. } => Some(ValueType::Bits(Width::Fixed(*width))),

        Expr::ReadSpecial { width, .. } => Some(ValueType::Bits(Width::Fixed(*width))),

        Expr::SystemReg { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W64))),

        Expr::ReadFlag { .. } => Some(ValueType::Bool),

        Expr::Binary { ty, .. }
        | Expr::Unary { ty, .. }
        | Expr::Select { ty, .. }
        | Expr::TupleElement { ty, .. } => Some(*ty),

        Expr::Shift { ty, .. } => Some(ValueType::Bits(Width::Fixed(*ty))),

        Expr::ScalarFPBinary { ty, .. } => Some(ValueType::Bits(Width::Fixed(*ty))),

        Expr::Extend { to, .. } | Expr::ZeroExtend { to, .. } | Expr::SignExtend { to, .. } => {
            Some(ValueType::Bits(Width::Fixed(*to)))
        }

        Expr::BitTest { .. } => Some(ValueType::Bool),
        Expr::ExtendReg { to, .. } => Some(ValueType::Bits(*to)),

        Expr::Extract { width, .. } => Some(ValueType::Bits(Width::Fixed(*width))),

        Expr::Concat { width, .. } => Some(ValueType::Bits(Width::Fixed(*width))),

        Expr::Replicate {
            count,
            element_width,
            ..
        } => {
            let total = u32::from(*count) * u32::from(element_width.bits());

            if total == 0 || total > 128 {
                None
            } else {
                BitWidth::new(total as u16).map(|width| ValueType::Bits(Width::Fixed(width)))
            }
        }

        Expr::VectorMap {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        Expr::VectorMap2 {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        Expr::VectorReduce {
            esize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*esize))),

        Expr::VectorPairwise {
            result_datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*result_datasize))),

        Expr::VectorNarrowing {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        Expr::VectorPerm {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        // VectorDeinterleave produces two destination vectors (each VL bits)
        // The expression represents the pair of results, so we use a tuple-like approach
        // For validation purposes, we consider it produces the source datasize (VL bits)
        Expr::VectorDeinterleave {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        // VectorInterleave produces a single result vector (VL bits with 16-bit elements)
        Expr::VectorInterleave {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        Expr::VectorDot {
            datasize,
            ..
        } => Some(ValueType::Bits(Width::Fixed(*datasize))),

        Expr::Condition(_) | Expr::ConditionHolds { .. } => Some(ValueType::Bool),

        // MTE tag selection and tag loads always yield a 4-bit allocation tag.
        Expr::ChooseTag { .. } | Expr::ChooseTagOrZero { .. } | Expr::MemTagRead { .. } => {
            Some(ValueType::Bits(Width::Fixed(BitWidth::W4)))
        }

        // Checked-pointer adjustment preserves 64-bit addresses.
        Expr::PointerAddCheck { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W64))),

        // A system op is a statement-level effect with no scalar value.
        Expr::SysOp { .. } => None,

        // SYSL returns a 64-bit result in Xt.
        Expr::SysLOp { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W64))),

        // PAC authentication returns 64-bit authenticated address.
        Expr::AuthDA { .. }
        | Expr::AuthIA { .. }
        | Expr::AuthIB { .. }
        | Expr::AuthDB { .. }
        | Expr::AuthIA2 { .. }
        | Expr::AuthIB2 { .. }
        | Expr::AddPAC { .. }
        | Expr::AddPAC2 { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W64))),

        // SVE expression types
        Expr::ReadPredReg { pl, .. } => Some(ValueType::Bits(Width::Fixed(*pl))),
        Expr::ReadScalableVecReg { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::WritePredReg { .. } => None, // Statement-level
        Expr::WriteScalableVecReg { .. } => None, // Statement-level
        Expr::PredicateMap { pl, .. } => Some(ValueType::Bits(Width::Fixed(*pl))),
        Expr::ScalableVectorMap { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::ScalableVectorMap2 { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::ScalableVectorMap3 { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::ScalableVectorPairwise { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::PredicateReduce { .. } => Some(ValueType::Bool), // Reduces to condition flags
        // SVE vector reductions produce scalar results of varying sizes
        // Floating-point reductions (FMaxV, FMinV, FMaxNV, FMinNV, FMaxQV, FMinQV, FMaxNQV, FMinNQV) return esize (32 or 64 bits)
        // Integer reductions to X registers return 64 bits
        Expr::ScalableVectorReduce { op, esize, .. } => {
            let is_fp_reduction = matches!(
                *op,
                SVEVectorReduceOp::FMaxV
                    | SVEVectorReduceOp::FMinV
                    | SVEVectorReduceOp::FMaxNV
                    | SVEVectorReduceOp::FMinNV
                    | SVEVectorReduceOp::FMaxQV
                    | SVEVectorReduceOp::FMinQV
                    | SVEVectorReduceOp::FMaxNQV
                    | SVEVectorReduceOp::FMinNQV
                    | SVEVectorReduceOp::FAddA  // FADDA: strictly-ordered reduction with scalar initial value
            );
            if is_fp_reduction {
                Some(ValueType::Bits(Width::Fixed(*esize)))
            } else {
                Some(ValueType::Bits(Width::Fixed(BitWidth::W64)))
            }
        }
        Expr::SVEGatherScatter { vl, .. } => Some(ValueType::Bits(Width::Fixed(*vl))),
        Expr::SVESpecial { vl, op, .. } => {
            // RDVL, FIRSTP, DecodePredCount, CountPred return 64-bit scalar
            // CounterToPredicate returns predicate mask (PL*4 bits)
            // FAddQVSeg returns 128-bit vector (quadword segment result)
            // LastA_Scalar, LastB_Scalar, LastP return 64-bit scalar (for X register)
            // LastA, LastB (SIMD&FP) return esize bits (for V register)
            // Clasta, Clastb return 64-bit scalar
            // Match, Nmatch return predicate (PL bits where PL = VL/8)
            // Rdffr, Rdffrs return predicate (PL bits where PL = VL/8)
            // other special ops return VL bits
            if matches!(*op, SVESpecialOp::Rdvl | SVESpecialOp::FirstP | SVESpecialOp::DecodePredCount | SVESpecialOp::CountPred | SVESpecialOp::LastP | SVESpecialOp::LastA_Scalar | SVESpecialOp::LastB_Scalar | SVESpecialOp::Clasta | SVESpecialOp::Clastb) {
                Some(ValueType::Bits(Width::Fixed(BitWidth::W64)))
            } else if matches!(*op, SVESpecialOp::CounterToPredicate) {
                // CounterToPredicate produces predicate mask (PL*4 bits)
                let pl_bits = vl.bits() / 8 * 4;
                crate::types::BitWidth::new(pl_bits as u16).map(|pl| ValueType::Bits(Width::Fixed(pl)))
            } else if matches!(*op, SVESpecialOp::FAddQVSeg) {
                Some(ValueType::Bits(Width::Fixed(BitWidth::W128)))
            } else if matches!(*op, SVESpecialOp::Match | SVESpecialOp::Nmatch | SVESpecialOp::Rdffr | SVESpecialOp::Rdffrs | SVESpecialOp::Setffr | SVESpecialOp::Wrffr | SVESpecialOp::Whilelt | SVESpecialOp::Whilele | SVESpecialOp::Whilelo | SVESpecialOp::Whilels | SVESpecialOp::Whilegt | SVESpecialOp::Whilege | SVESpecialOp::Whilehi | SVESpecialOp::Whilehs | SVESpecialOp::Whilerw | SVESpecialOp::Whilewr) {
                // Match, Nmatch, Rdffr, Rdffrs, Setffr, Wrffr, Whilelt, Whilele, Whilelo, Whilels, Whilegt, Whilege, Whilehi, Whilehs, Whilerw, Whilewr produce predicate results (PL = VL/8 bits)
                let pl_bits = vl.bits() / 8;
                crate::types::BitWidth::new(pl_bits as u16).map(|pl| ValueType::Bits(Width::Fixed(pl)))
            } else {
                Some(ValueType::Bits(Width::Fixed(*vl)))
            }
        }
        Expr::PredicateMap2 { pl, .. } => Some(ValueType::Bits(Width::Fixed(*pl))),

        // Feature check returns 64-bit value.
        Expr::ChkFeat { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W64))),

        // CRC checksum always returns a 32-bit result (W register).
        Expr::Crc32 { .. } => Some(ValueType::Bits(Width::Fixed(BitWidth::W32))),

        // AddWithCarry is a two-element tuple:
        //
        // (result: bits(width), nzcv: bits(4))
        //
        // Therefore it does not itself have a scalar ValueType.
        Expr::AddWithCarry { .. } => None,

        Expr::If { .. } => None, // Statement-level, no scalar value
        Expr::Call { .. } => None, // Statement-level, no scalar value
        Expr::FieldAccess { base, field } => {
            // If base is PSTATE, return appropriate type for the field
            if let Expr::FieldAccess { field: base_field, .. } = &**base {
                if base_field == "PSTATE" {
                    return match field.as_str() {
                        "N" | "Z" | "C" | "V" | "SP" => Some(ValueType::Bool),
                        "EL" => Some(ValueType::Bits(Width::Fixed(BitWidth::W2))),
                        _ => None, // Unknown field
                    };
                }
            }
            None
        }
        Expr::EnumLit { .. } => None, // Enum literal - no scalar value
        Expr::Slice { .. } => None, // Slice - handled by Extract/Concat in practice
        Expr::GenericPrim { .. } => None, // Generic primitive - type varies

        Expr::MemRead { size, .. } | Expr::MemAtomic { size, .. } | Expr::MemAtomicRMW { size, .. } => {
            Some(ValueType::Bits(Width::Fixed(size.width())))
        }
    }
}
