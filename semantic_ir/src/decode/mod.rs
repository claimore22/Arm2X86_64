pub mod adc_sbc;
pub mod add_sub;
pub mod addsub_flags;
pub mod addsub_imm;
pub mod adr_adrp;
pub mod barrier;
pub mod bitfield;
pub mod branch_imm;
pub mod branch_reg;
pub mod cbz_cbnz;
pub mod ccmp;
pub mod cond_select;
pub mod divide;
pub mod extend;
pub mod extr;
pub mod hint;
pub mod msr_mrs;
pub mod logical;
pub mod logical_imm;
pub mod move_wide;
pub mod multiply;
pub mod multiply_long;
pub mod tbz_tbnz;
pub mod unary;

use crate::expr::{BinOp, Expr, UnaryOp};
use crate::instruction::{Instruction, InstructionId};
use crate::primitives::{Condition, ExtendKind, ShiftKind};
use crate::stmt::Stmt;
use crate::types::{BitWidth, Flag, Reg, ValueType, Width};
use crate::Provenance;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UndefinedInstruction,
    InvalidShift,
    InvalidRegister,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub mnemonic: String,
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub shift: ShiftKind,
    pub shift_amount: u32,
    pub imm: u32,
    pub n: u8,
    pub width: BitWidth,
}

pub fn decode_instruction(bits: u32) -> Result<DecodedInstruction, DecodeError> {
    let t = (bits >> 10) & 0x3F;
    
    // CCMP/CCMN: bits 28-24 = 11010 (0x1A), bit 28 = 1, bit 29 = 1 (S=1), bit 23 = 0
    let is_ccmp = ((bits >> 28) & 1) == 1 && ((bits >> 29) & 1) == 1 && ((bits >> 24) & 0x1F) == 0x1A && ((bits >> 23) & 1) == 0 && t != 0x02 && t != 0x03;
    
    // ADC/SBC: bits 28-24 = 11010 (0x1A), bit 28 = 1, bit 29 = 0 (S=0), bit 23 = 0
    let is_adc_sbc = ((bits >> 24) & 0x1F) == 0x1A && ((bits >> 29) & 1) == 0 && ((bits >> 23) & 1) == 0;
    
    // ADD/SUB (shifted register): bits 28-24 = 10010 (0x12), bit 28 = 1, bit 23 = 0, bits 22-20 = 110 (0x6)
    // Standard encoding: 0x92020020 for ADD, 0xD2020020 for SUB
    let is_add_sub = ((bits >> 24) & 0x1F) == 0x12 && ((bits >> 23) & 1) == 0 && ((bits >> 20) & 0x7) == 0x6;
    
    // Logical shifted: bits 28-24 = 10100 (0x14), bit 28 = 0, bit 23 = 0
    // This matches compiler output for AND/ORR/EOR/ANDS (0x94, 0xB4, 0xD4, 0xF4)
    let is_logical = (((bits >> 24) & 0x1F) == 0x14 || ((bits >> 24) & 0x1F) == 0x0A) && ((bits >> 23) & 1) == 0;
    let is_move_wide = ((bits >> 28) & 1) == 1
        && ((bits >> 24) & 0x0F) == 0b0010
        && ((bits >> 23) & 1) == 1;
    let is_bitfield = ((bits >> 24) & 0x1F) == 0b10011;
    let is_addsub_imm = ((bits >> 24) & 0x1F) == 0b10001;
    let is_extend = ((bits >> 24) & 0x1F) == 0x0B && ((bits >> 29) & 1) == 0;
    // Data Processing - 1 Source (REV, REV16, REV32, REV64, CLZ, CLS, RBIT)
// bits 27-23 = 10101 (0x15), bits 22-19 = 1100 (0xC)
    let is_unary = ((bits >> 23) & 0x1F) == 0b10101
        && ((bits >> 19) & 0xF) == 0xC;
    // EXTR (Extract register): bits 28-22 = 110001111 (0x18F), N=0, opc=11
    // bits 31-22: SF 11 0 001111
    let is_extr = ((bits >> 22) & 0x1FF) == 0b110001111 && ((bits >> 28) & 1) == 0 && ((bits >> 29) & 3) == 3;
    // HINT: bits 31-8 = 0xD50320
    let is_hint = (bits >> 8) == 0xD50320;
    // Barrier: ISB/DSB/DMB - bits 31-20 = 0xD5033, bits 19-16 = 0x3, CRn = 3, op2 = 4/5/6, Rt = 0x1F
    let is_barrier = ((bits >> 24) & 0xFF) == 0xD5
        && ((bits >> 20) & 0xF) == 0
        && ((bits >> 16) & 7) == 3
        && ((bits >> 12) & 0xF) == 3
        && ((bits >> 5) & 7) >= 4 && ((bits >> 5) & 7) <= 6
        && (bits & 0x1F) == 0x1F;
    // MSR/MRS (system register): bits 31-21 = 1101010100x (x=0 for MSR reg, x=1 for MRS)
    // MRS: 0xD53... (bits 31-21 = 11010101011 = 0xD53)
    // MSR (register): 0xD51... (bits 31-21 = 11010101001 = 0xD51)
    let is_msr_mrs = ((bits >> 21) & 0x7FF) == 0b11010101000 || ((bits >> 21) & 0x7FF) == 0b11010101001;
    // Load/Store (unsigned offset): bits 29-24 = 111001 (0x39)
    // TBZ/TBNZ: bits 30-24 = 011011x (0x36 for TBZ), 0110111 (0x37 for TBNZ)
        let is_tbz_tbnz = ((bits >> 24) & 0x7F) == 0x36 || ((bits >> 24) & 0x7F) == 0x37;
    let is_adr_adrp = ((bits >> 24) & 0xFF) == 0x10 || ((bits >> 24) & 0xFF) == 0x90;
    // Branch register: bits 31-24 = 11010110 (0xD6), bits 20-10 = 11111000000 (0x7C0), bit 4 = 0
    let is_branch_reg = ((bits >> 24) & 0xFF) == 0xD6
        && ((bits >> 10) & 0x7FF) == 0x7C0
        && ((bits >> 4) & 1) == 0;
    // Branch immediate: B/BL/B.cond
    // B: bits 31-26 = 000101 (0x05)
    // BL: bits 31-26 = 100101 (0x25)
    // B.cond: bits 31-24 = 01010100 (0x54)
    let is_branch_imm = ((bits >> 26) & 0x3F) == 0b000101
        || ((bits >> 26) & 0x3F) == 0b100101
        || ((bits >> 24) & 0xFF) == 0x54;
    let is_addsub_imm = ((bits >> 24) & 0x1F) == 0b10001;
    let is_extend = ((bits >> 24) & 0x1F) == 0x0B && ((bits >> 29) & 1) == 0;
    let is_addsub_flags = ((bits >> 24) & 0x1F) == 0b01011 && ((bits >> 29) & 1) == 1;
    let is_cond_select = ((bits >> 24) & 0x1F) == 0b11010 && ((bits >> 23) & 1) == 1 && !(( (bits >> 10) & 0x3F == 0x08) || ((bits >> 10) & 0x3F == 0x0C));
    let is_divide = ((bits >> 24) & 0x1F) == 0b11010 && ((bits >> 23) & 1) == 1 && (t == 0x02 || t == 0x03);
    let is_logical_imm = ((bits >> 20) & 0xFF) == 0x24 && ((bits >> 23) & 1) == 0;
    // Multiply long/high: bits 28-24 = 11011 (0x1B), bit 23 = 0, bits 22-21 != 0
    // SMADDL/UMADDL/SMULL/UMULL (bits 22-21 = 01), SMULH/UMULH (bits 22-21 = 10)
    // Signedness is bit 23; allow both signed and unsigned.
    let is_multiply_long = ((bits >> 24) & 0x1F) == 0x1B && (((bits >> 21) & 3) != 0);
    // Regular multiply: bits 28-24 = 11011 (0x1B), bit 23 = 0, bits 22-21 = 00
    let is_multiply = ((bits >> 24) & 0x1F) == 0x1B && ((bits >> 23) & 1) == 0 && (((bits >> 21) & 3) == 0);
    let is_logical_imm = ((bits >> 20) & 0xFF) == 0x24 && ((bits >> 23) & 1) == 0;
    
    if is_ccmp {
        ccmp::decode_ccmp(bits)
    } else if is_adc_sbc {
        adc_sbc::decode_adc_sbc(bits)
    } else if is_divide {
        divide::decode_divide(bits)
    } else if is_add_sub {
        add_sub::decode_add_sub(bits)
    } else if is_logical {
        logical::decode_logical(bits)
    } else if is_move_wide {
        move_wide::decode_move_wide(bits)
    } else if is_bitfield {
        bitfield::decode_bitfield(bits)
    } else if is_unary {
        unary::decode_unary(bits)
    } else if is_extr {
        extr::decode_extr(bits)
    } else if is_hint {
        hint::decode_hint(bits)
    } else if is_tbz_tbnz {
        tbz_tbnz::decode_tbz_tbnz(bits)
    } else if is_adr_adrp {
        adr_adrp::decode_adr_adrp(bits)
    } else if is_barrier {
        barrier::decode_barrier(bits)
    } else if is_msr_mrs {
        msr_mrs::decode_msr_mrs(bits)
    } else if is_branch_reg {
        branch_reg::decode_branch_reg(bits)
    } else if is_branch_imm {
        branch_imm::decode_branch_imm(bits)
    } else if is_addsub_imm {
        addsub_imm::decode_addsub_imm(bits)
    } else if is_addsub_flags {
        addsub_flags::decode_addsub_flags(bits)
    } else if is_cond_select {
        cond_select::decode_cond_select(bits)
    } else if is_multiply_long {
        multiply_long::decode_multiply_long(bits)
    } else if is_multiply {
        multiply::decode_multiply(bits)
    } else if is_logical_imm {
        logical_imm::decode_logical_imm(bits)
    } else if is_addsub_flags {
        addsub_flags::decode_addsub_flags(bits)
    } else if is_extend {
        extend::decode_extend(bits)
    } else {
        add_sub::decode_add_sub(bits)
    }
}

pub fn lower_to_ir(decoded: &DecodedInstruction) -> Instruction {
      if decoded.mnemonic == "ADDX" || decoded.mnemonic == "SUBX" {
          let rm_width = match decoded.rm {
              Reg::W(_) | Reg::Wzr => BitWidth::W32,
              _ => BitWidth::W64,
          };
          let kind = match decoded.imm {
              0 => ExtendKind::Uxtb,
              1 => ExtendKind::Uxth,
              2 => ExtendKind::Uxtw,
              3 => ExtendKind::Uxtx,
              4 => ExtendKind::Sxtb,
              5 => ExtendKind::Sxth,
              6 => ExtendKind::Sxtw,
              _ => ExtendKind::Sxtx,
          };
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm_read = Expr::ReadReg { reg: decoded.rm, width: rm_width };
          let extended = Expr::ExtendReg {
              register: Box::new(rm_read),
              kind,
              shift: Box::new(Expr::Const { value: decoded.shift_amount as u128, ty: rm_width }),
              to: Width::Fixed(decoded.width),
          };
          let bin_op = if decoded.mnemonic == "SUBX" { BinOp::Sub } else { BinOp::Add };
          let result = Expr::Binary {
              op: bin_op,
              left: Box::new(rn),
              right: Box::new(extended),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          };
          let semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result }];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (ext)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "extend.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
      if decoded.mnemonic == "CCMP" || decoded.mnemonic == "CCMN" {
          let cond = match decoded.shift_amount {
              0 => Condition::Eq,
              1 => Condition::Ne,
              2 => Condition::Cs,
              3 => Condition::Cc,
              4 => Condition::Mi,
              5 => Condition::Pl,
              6 => Condition::Vs,
              7 => Condition::Vc,
              8 => Condition::Hi,
              9 => Condition::Ls,
              10 => Condition::Ge,
              11 => Condition::Lt,
              12 => Condition::Gt,
              13 => Condition::Le,
              _ => Condition::Al,
          };
          let nzcv = decoded.imm;
          let n_val = (nzcv >> 3) & 1 == 1;
          let z_val = (nzcv >> 2) & 1 == 1;
          let c_val = (nzcv >> 1) & 1 == 1;
          let v_val = nzcv & 1 == 1;
  
          // If condition holds: compute rn - rm and set flags; else set nzcv.
          // We'll use Match on ConditionHolds.
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let diff = Expr::Binary { op: BinOp::Sub, left: Box::new(rn.clone()), right: Box::new(rm.clone()), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) };
          let n_shift = Expr::Shift { kind: ShiftKind::Lsr, value: Box::new(diff.clone()), amount: 
Box::new(Expr::Const { value: (decoded.width.bits()-1) as u128, ty: decoded.width }), ty: decoded.width };
          let n_bool = Expr::Binary { op: BinOp::Eq, left: Box::new(n_shift), right: Box::new(Expr::Const { value: 1, 
ty: decoded.width }), ty: ValueType::Bool };
          let z_bool = Expr::Binary { op: BinOp::Eq, left: Box::new(diff), right: Box::new(Expr::Const { value: 0, ty: 
decoded.width }), ty: ValueType::Bool };
  
          let cond_holds = Expr::ConditionHolds { condition: cond };
  
          // Compute flags for the true branch (condition holds)
          // For CCMP: rn - rm, C = NOT borrow, V = signed overflow
          // For CCMN: rn + rm, C = carry out, V = signed overflow
          let (c_expr, v_expr) = if decoded.mnemonic == "CCMP" {
              // CCMP: rn - rm = rn + (~rm + 1), C = NOT borrow = carry out
              let not_rm = Expr::Unary {
                  op: UnaryOp::Not,
                  value: Box::new(rm.clone()),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn.clone()),
                  right: Box::new(not_rm),
                  carry: Box::new(Expr::Bool(true)),
                  width: decoded.width,
              };
              let c_elem = Expr::TupleElement {
                  value: Box::new(add_carry.clone()),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let c_bit = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(c_elem.clone()),
                  right: Box::new(Expr::Const { value: 0b10, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let c_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(c_bit), right: Box::new(Expr::Const { value: 
0, ty: BitWidth::W4 }), ty: ValueType::Bool };
              
              let v_elem = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let v_bit = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(v_elem),
                  right: Box::new(Expr::Const { value: 0b01, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let v_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(v_bit), right: Box::new(Expr::Const { value: 
0, ty: BitWidth::W4 }), ty: ValueType::Bool };
              
              (c_bool, v_bool)
          } else {
              // CCMN: rn + rm
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn.clone()),
                  right: Box::new(rm.clone()),
                  carry: Box::new(Expr::Bool(false)),
                  width: decoded.width,
              };
              let c_elem = Expr::TupleElement {
                  value: Box::new(add_carry.clone()),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let c_bit = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(c_elem.clone()),
                  right: Box::new(Expr::Const { value: 0b10, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let c_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(c_bit), right: Box::new(Expr::Const { value: 
0, ty: BitWidth::W4 }), ty: ValueType::Bool };
              
              let v_elem = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let v_bit = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(v_elem),
                  right: Box::new(Expr::Const { value: 0b01, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              let v_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(v_bit), right: Box::new(Expr::Const { value: 
0, ty: BitWidth::W4 }), ty: ValueType::Bool };
              
              (c_bool, v_bool)
          };
  
          let true_flags = vec![
              Stmt::WriteFlag { flag: Flag::N, value: n_bool },
              Stmt::WriteFlag { flag: Flag::Z, value: z_bool },
              Stmt::WriteFlag { flag: Flag::C, value: c_expr },
              Stmt::WriteFlag { flag: Flag::V, value: v_expr },
          ];
          let false_flags = vec![
              Stmt::WriteFlag { flag: Flag::N, value: Expr::Bool(n_val) },
              Stmt::WriteFlag { flag: Flag::Z, value: Expr::Bool(z_val) },
              Stmt::WriteFlag { flag: Flag::C, value: Expr::Bool(c_val) },
              Stmt::WriteFlag { flag: Flag::V, value: Expr::Bool(v_val) },
          ];
  
          let semantics = vec![Stmt::Match {
              scrutinee: cond_holds,
              arms: vec![
                  crate::stmt::MatchArm { patterns: vec![crate::stmt::MatchPattern::Constant(Expr::Bool(true))], 
guard: None, body: true_flags },
                  crate::stmt::MatchArm { patterns: vec![crate::stmt::MatchPattern::Constant(Expr::Bool(false))], 
guard: None, body: false_flags },
              ],
          }];
  
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (cond cmp)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "ccmp.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "ADDS" || decoded.mnemonic == "SUBS" {
          let rn_expr = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm_expr = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let shift_amount = Expr::Const { value: decoded.shift_amount as u128, ty: decoded.width };
          let shifted = Expr::Shift { kind: decoded.shift, value: Box::new(rm_expr), amount: Box::new(shift_amount), 
ty: decoded.width };
          
          let (result, carry_expr) = if decoded.mnemonic == "SUBS" {
              // SUBS: Rn - shifted = Rn + (~shifted + 1) = AddWithCarry(Rn, ~shifted, carry_in=1)
              // C flag = NOT borrow = carry out from Rn + (~shifted) + 1
              let not_shifted = Expr::Unary {
                  op: UnaryOp::Not,
                  value: Box::new(shifted.clone()),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let carry_in = Expr::Bool(true); // carry_in = 1 for subtraction
              let not_shifted_for_carry = not_shifted.clone();
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn_expr.clone()),
                  right: Box::new(not_shifted),
                  carry: Box::new(carry_in),
                  width: decoded.width,
              };
              // Extract result (tuple element 0)
              let result = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 0,
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              // Extract NZCV flags (tuple element 1)
              let carry_expr = Expr::TupleElement {
                  value: Box::new(Expr::AddWithCarry {
                      left: Box::new(rn_expr),
                      right: Box::new(not_shifted_for_carry),
                      carry: Box::new(Expr::Bool(true)),
                      width: decoded.width,
                  }),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)), // NZCV packed in 4 bits
              };
              (result, carry_expr)
          } else {
              // ADDS: Rn + shifted, C = carry out, V = signed overflow
              let shifted_for_carry = shifted.clone();
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn_expr.clone()),
                  right: Box::new(shifted),
                  carry: Box::new(Expr::Bool(false)),
                  width: decoded.width,
              };
              let result = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 0,
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let carry_expr = Expr::TupleElement {
                  value: Box::new(Expr::AddWithCarry {
                      left: Box::new(rn_expr),
                      right: Box::new(shifted_for_carry),
                      carry: Box::new(Expr::Bool(false)),
                      width: decoded.width,
                  }),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              (result, carry_expr)
          };
          
          let rd_read = Expr::ReadReg { reg: decoded.rd, width: decoded.width };
          let n_shift = Expr::Shift { kind: ShiftKind::Lsr, value: Box::new(rd_read.clone()), amount: 
Box::new(Expr::Const { value: (decoded.width.bits()-1) as u128, ty: decoded.width }), ty: decoded.width };
          let n_bool = Expr::Binary { op: BinOp::Eq, left: Box::new(n_shift), right: Box::new(Expr::Const { value: 1, 
ty: decoded.width }), ty: ValueType::Bool };
          let z_expr = Expr::Binary { op: BinOp::Eq, left: Box::new(rd_read), right: Box::new(Expr::Const { value: 0, 
ty: decoded.width }), ty: ValueType::Bool };
          
          // Extract C and V from NZCV tuple (bit 1 = C, bit 0 = V)
          let c_expr = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr.clone()),
              right: Box::new(Expr::Const { value: 0b10, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let c_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(c_expr), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let v_expr = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr),
              right: Box::new(Expr::Const { value: 0b01, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let v_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(v_expr), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let semantics = vec![
              Stmt::WriteReg { reg: decoded.rd, value: result },
              Stmt::WriteFlag { flag: Flag::N, value: n_bool },
              Stmt::WriteFlag { flag: Flag::Z, value: z_expr },
              Stmt::WriteFlag { flag: Flag::C, value: c_bool },
              Stmt::WriteFlag { flag: Flag::V, value: v_bool },
          ];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (flags)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "addsub_flags.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
      if decoded.mnemonic == "ANDI" || decoded.mnemonic == "ORRI" || decoded.mnemonic == "EORI" || decoded.mnemonic == 
"ANDSI" {
          let n_bit = decoded.n == 1;
          let immr = decoded.shift_amount as u8;
          let imms = decoded.imm as u8;
          let (imm_mask, _) = crate::shared::decode_bit_masks::<u8, u64>(n_bit, imms, immr, true);
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let bin_op = match decoded.mnemonic.as_str() {
              "ORRI" => BinOp::Or,
              "EORI" => BinOp::Xor,
              _ => BinOp::And,
          };
          let result = Expr::Binary {
              op: bin_op,
              left: Box::new(rn),
              right: Box::new(Expr::Const { value: imm_mask as u128, ty: decoded.width }),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          };
          let mut semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result.clone() }];
          if decoded.mnemonic == "ANDSI" {
              let rd_read = Expr::ReadReg { reg: decoded.rd, width: decoded.width };
              let n_shift = Expr::Shift { kind: ShiftKind::Lsr, value: Box::new(rd_read.clone()), amount: 
Box::new(Expr::Const { value: (decoded.width.bits() - 1) as u128, ty: decoded.width }), ty: decoded.width };
              let n_bool = Expr::Binary { op: BinOp::Eq, left: Box::new(n_shift), right: Box::new(Expr::Const { value: 
1, ty: decoded.width }), ty: ValueType::Bool };
              let z_expr = Expr::Binary { op: BinOp::Eq, left: Box::new(rd_read), right: Box::new(Expr::Const { value: 
0, ty: decoded.width }), ty: ValueType::Bool };
              semantics.push(Stmt::WriteFlag { flag: Flag::N, value: n_bool });
              semantics.push(Stmt::WriteFlag { flag: Flag::Z, value: z_expr });
              semantics.push(Stmt::WriteFlag { flag: Flag::C, value: Expr::Bool(false) });
              semantics.push(Stmt::WriteFlag { flag: Flag::V, value: Expr::Bool(false) });
          }
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (imm)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "logical_imm.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "SDIV" || decoded.mnemonic == "UDIV" {
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let op = if decoded.mnemonic == "SDIV" { BinOp::Div } else { BinOp::UDiv };
          let result = Expr::Binary { op, left: Box::new(rn), right: Box::new(rm), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) };
          let semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result }];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (div)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "div.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "MUL" || decoded.mnemonic == "MNEG" || decoded.mnemonic == "MADD" || decoded.mnemonic == 
"MSUB" {
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let prod = Expr::Binary { op: BinOp::Mul, left: Box::new(rn), right: Box::new(rm), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) };
          let result = match decoded.mnemonic.as_str() {
              "MUL" => prod,
              "MNEG" => Expr::Unary { op: UnaryOp::Neg, value: Box::new(prod), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) },
              "MADD" => {
                  let ra = Expr::ReadReg { reg: Reg::x(decoded.shift_amount as u8).unwrap_or(Reg::Xzr), width: 
decoded.width };
                  Expr::Binary { op: BinOp::Add, left: Box::new(ra), right: Box::new(prod), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) }
              }
              "MSUB" => {
                  let ra = Expr::ReadReg { reg: Reg::x(decoded.shift_amount as u8).unwrap_or(Reg::Xzr), width: 
decoded.width };
                  Expr::Binary { op: BinOp::Sub, left: Box::new(ra), right: Box::new(prod), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) }
              }
              _ => prod,
          };
          let semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result }];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (mul)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "mul.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "ADC" || decoded.mnemonic == "SBC" {
          let rn_expr = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm_expr = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let carry_flag = Expr::ReadFlag { flag: Flag::C };
          
          // Use AddWithCarry to compute result and flags, similar to ADDS/SUBS
          let (result, carry_expr) = if decoded.mnemonic == "SBC" {
              // SBC: Rn - Rm - (NOT C) = Rn + (~Rm + C)
              // C flag = NOT borrow = carry out from Rn + (~Rm) + C
              let not_rm = Expr::Unary {
                  op: UnaryOp::Not,
                  value: Box::new(rm_expr.clone()),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let carry_in = Expr::Select {
                  condition: Box::new(carry_flag.clone()),
                  when_true: Box::new(Expr::Bool(true)),
                  when_false: Box::new(Expr::Bool(false)),
                  ty: ValueType::Bool,
              };
              let not_rm_for_carry = not_rm.clone();
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn_expr.clone()),
                  right: Box::new(not_rm),
                  carry: Box::new(carry_in),
                  width: decoded.width,
              };
              let result = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 0,
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let carry_expr = Expr::TupleElement {
                  value: Box::new(Expr::AddWithCarry {
                      left: Box::new(rn_expr.clone()),
                      right: Box::new(not_rm_for_carry),
                      carry: Box::new(carry_flag),
                      width: decoded.width,
                  }),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              (result, carry_expr)
          } else {
              // ADC: Rn + Rm + C
              let carry_in = Expr::Select {
                  condition: Box::new(carry_flag.clone()),
                  when_true: Box::new(Expr::Bool(true)),
                  when_false: Box::new(Expr::Bool(false)),
                  ty: ValueType::Bool,
              };
              let rm_for_carry = rm_expr.clone();
              let add_carry = Expr::AddWithCarry {
                  left: Box::new(rn_expr.clone()),
                  right: Box::new(rm_expr.clone()),
                  carry: Box::new(carry_in),
                  width: decoded.width,
              };
              let result = Expr::TupleElement {
                  value: Box::new(add_carry),
                  index: 0,
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let carry_expr = Expr::TupleElement {
                  value: Box::new(Expr::AddWithCarry {
                      left: Box::new(rn_expr.clone()),
                      right: Box::new(rm_for_carry),
                      carry: Box::new(carry_flag),
                      width: decoded.width,
                  }),
                  index: 1,
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              };
              (result, carry_expr)
          };
          
          // Extract flags from NZCV tuple (bits: N=3, Z=2, C=1, V=0)
          let n_elem = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr.clone()),
              right: Box::new(Expr::Const { value: 0b1000, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let n_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(n_elem), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let z_elem = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr.clone()),
              right: Box::new(Expr::Const { value: 0b0100, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let z_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(z_elem), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let c_elem = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr.clone()),
              right: Box::new(Expr::Const { value: 0b0010, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let c_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(c_elem), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let v_elem = Expr::Binary {
              op: BinOp::And,
              left: Box::new(carry_expr),
              right: Box::new(Expr::Const { value: 0b0001, ty: BitWidth::W4 }),
              ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
          };
          let v_bool = Expr::Binary { op: BinOp::Ne, left: Box::new(v_elem), right: Box::new(Expr::Const { value: 0, 
ty: BitWidth::W4 }), ty: ValueType::Bool };
          
          let semantics = vec![
              Stmt::WriteReg { reg: decoded.rd, value: result },
              Stmt::WriteFlag { flag: Flag::N, value: n_bool },
              Stmt::WriteFlag { flag: Flag::Z, value: z_bool },
              Stmt::WriteFlag { flag: Flag::C, value: c_bool },
              Stmt::WriteFlag { flag: Flag::V, value: v_bool },
          ];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (carry)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "adc_sbc.md".to_string(), source_sha256: "t".to_string(), 
clean_sha256: "t".to_string() },
          };
      }
      if decoded.mnemonic == "CSEL" || decoded.mnemonic == "CSINC" || decoded.mnemonic == "CSINV" || decoded.mnemonic 
== "CSNEG" {
          let cond = match decoded.imm {
              0 => Condition::Eq,
              1 => Condition::Ne,
              2 => Condition::Cs,
              3 => Condition::Cc,
              4 => Condition::Mi,
              5 => Condition::Pl,
              6 => Condition::Vs,
              7 => Condition::Vc,
              8 => Condition::Hi,
              9 => Condition::Ls,
              10 => Condition::Ge,
              11 => Condition::Lt,
              12 => Condition::Gt,
              13 => Condition::Le,
              _ => Condition::Al,
          };
          let rn = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let rm = Expr::ReadReg { reg: decoded.rm, width: decoded.width };
          let when_true = match decoded.mnemonic.as_str() {
              "CSINC" => Expr::Binary { op: BinOp::Add, left: Box::new(rn.clone()), right: Box::new(Expr::Const { 
value: 1, ty: decoded.width }), ty: ValueType::Bits(Width::Fixed(decoded.width)) },
              "CSINV" => Expr::Unary { op: UnaryOp::Not, value: Box::new(rn.clone()), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) },
              "CSNEG" => Expr::Unary { op: UnaryOp::Neg, value: Box::new(rn.clone()), ty: 
ValueType::Bits(Width::Fixed(decoded.width)) },
              _ => rn.clone(),
          };
          let cond_expr = Expr::ConditionHolds { condition: cond };
          let result = Expr::Select {
              condition: Box::new(cond_expr),
              when_true: Box::new(when_true),
              when_false: Box::new(rm),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          };
          let semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result }];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (cond select) {}", 
decoded.mnemonic, decoded.width.bits())) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "arm64_md_clean/csel.md".to_string(), source_sha256: 
"t".to_string(), clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "ADDI" || decoded.mnemonic == "SUBI" {
          let op = if decoded.mnemonic == "SUBI" { BinOp::Sub } else { BinOp::Add };
          let result = Expr::Binary {
              op,
              left: Box::new(Expr::ReadReg { reg: decoded.rn, width: decoded.width }),
              right: Box::new(Expr::Const { value: decoded.imm as u128, ty: decoded.width }),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          };
          let semantics = vec![Stmt::WriteReg { reg: decoded.rd, value: result }];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (imm) {}", 
decoded.mnemonic, decoded.width.bits())) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "arm64_md_clean/addsub_imm.md".to_string(), source_sha256: 
"t".to_string(), clean_sha256: "t".to_string() },
          };
      }
      
      if decoded.mnemonic == "ADDSI" || decoded.mnemonic == "SUBSI" {
          let width = decoded.width;
          let mask: u128 = if width.bits() >= 128 { !0u128 } else { (1u128 << width.bits()) - 1 };
          let rn_expr = Expr::ReadReg { reg: decoded.rn, width };
          let imm = decoded.imm as u128;
          let (right, carry) = if decoded.mnemonic == "SUBSI" {
              ((!imm) & mask, true)
          } else {
              (imm, false)
          };
          let add_carry = Expr::AddWithCarry {
              left: Box::new(rn_expr.clone()),
              right: Box::new(Expr::Const { value: right, ty: width }),
              carry: Box::new(Expr::Bool(carry)),
              width,
          };
          let result = Expr::TupleElement {
              value: Box::new(add_carry.clone()),
              index: 0,
              ty: ValueType::Bits(Width::Fixed(width)),
          };
          let rd_read = Expr::ReadReg { reg: decoded.rd, width };
          let n_shift = Expr::Shift {
              kind: ShiftKind::Lsr,
              value: Box::new(rd_read.clone()),
              amount: Box::new(Expr::Const { value: (width.bits() - 1) as u128, ty: width }),
              ty: width,
          };
          let n_bool = Expr::Binary {
              op: BinOp::Eq,
              left: Box::new(n_shift),
              right: Box::new(Expr::Const { value: 1, ty: width }),
              ty: ValueType::Bool,
          };
          let z_bool = Expr::Binary {
              op: BinOp::Eq,
              left: Box::new(rd_read),
              right: Box::new(Expr::Const { value: 0, ty: width }),
              ty: ValueType::Bool,
          };
          let c_bool = Expr::Binary {
              op: BinOp::Ne,
              left: Box::new(Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(Expr::TupleElement {
                      value: Box::new(add_carry.clone()),
                      index: 1,
                      ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
                  }),
                  right: Box::new(Expr::Const { value: 0b10, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              }),
              right: Box::new(Expr::Const { value: 0, ty: BitWidth::W4 }),
              ty: ValueType::Bool,
          };
          let v_bool = Expr::Binary {
              op: BinOp::Ne,
              left: Box::new(Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(Expr::TupleElement {
                      value: Box::new(add_carry),
                      index: 1,
                      ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
                  }),
                  right: Box::new(Expr::Const { value: 0b01, ty: BitWidth::W4 }),
                  ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
              }),
              right: Box::new(Expr::Const { value: 0, ty: BitWidth::W4 }),
              ty: ValueType::Bool,
          };
          let semantics = vec![
              Stmt::WriteReg { reg: decoded.rd, value: result },
              Stmt::WriteFlag { flag: Flag::N, value: n_bool },
              Stmt::WriteFlag { flag: Flag::Z, value: z_bool },
              Stmt::WriteFlag { flag: Flag::C, value: c_bool },
              Stmt::WriteFlag { flag: Flag::V, value: v_bool },
          ];
          return Instruction {
              id: InstructionId { mnemonic: decoded.mnemonic.clone(), encoding: Some(format!("{} (imm, flags)", 
decoded.mnemonic)) },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance { source_path: "arm64_md_clean/addsub_imm.md".to_string(), source_sha256: 
"t".to_string(), clean_sha256: "t".to_string() },
          };
      }
  
      if decoded.mnemonic == "SBFM" || decoded.mnemonic == "UBFM" || decoded.mnemonic == "BFM" {
          let n_bit = decoded.n == 1;
          let immr = decoded.shift_amount as u8;
          let imms = decoded.imm as u8;
          let (wmask, tmask) = crate::shared::decode_bit_masks::<u8, u64>(n_bit, imms, immr, false);
          let src = Expr::ReadReg { reg: decoded.rn, width: decoded.width };
          let w = decoded.width.bits() as u32;
          let r = decoded.shift_amount & 0x3F; // immr (6-bit)
          // bot = ROR(src, r) & wmask
          let ror_src = Expr::Shift {
              kind: ShiftKind::Ror,
              value: Box::new(src.clone()),
              amount: Box::new(Expr::Const { value: r as u128, ty: decoded.width }),
              ty: decoded.width,
          };
          let bot = Expr::Binary {
              op: BinOp::And,
              left: Box::new(ror_src),
              right: Box::new(Expr::Const { value: wmask as u128, ty: decoded.width }),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          };
  
          let result_expr = if decoded.mnemonic == "UBFM" {
              Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(bot),
                  right: Box::new(Expr::Const { value: tmask as u128, ty: decoded.width }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              }
          } else if decoded.mnemonic == "SBFM" {
              // top = sign replicate bit s. s = imms & levels. Compute s.
              // Replicate src[s]: (src << (w-1-s)) >> (w-1) arithmetic.
              // Need s = imms & levels. Recompute levels.
              let not_imms = imms ^ 0x3F;
              let imm_n_not_imms = (if n_bit { 1u8 << 5 } else { 0u8 }) | (not_imms & 0x3F);
              let len = 7 - imm_n_not_imms.leading_zeros() as u8;
              // levels = (1<<len)-1
              let levels = if len == 0 { 0u8 } else { (1u8 << len) - 1 };
              let s = imms & levels;
              let shift_left = (w as i32 - 1 - s as i32) as u32;
              let top = Expr::Shift {
                  kind: ShiftKind::Asr,
                  value: Box::new(Expr::Shift {
                      kind: ShiftKind::Lsl,
                      value: Box::new(src.clone()),
                      amount: Box::new(Expr::Const { value: shift_left as u128, ty: decoded.width }),
                      ty: decoded.width,
                  }),
                  amount: Box::new(Expr::Const { value: (w - 1) as u128, ty: decoded.width }),
                  ty: decoded.width,
              };
              let bot_masked = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(bot.clone()),
                  right: Box::new(Expr::Const { value: tmask as u128, ty: decoded.width }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let top_masked = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(top),
                  right: Box::new(Expr::Const { value: (!tmask) as u128 & (if decoded.width == BitWidth::W64 { 
u64::MAX as u128 } else { 0xFFFFFFFFu128 }), ty: decoded.width }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              Expr::Binary {
                  op: BinOp::Or,
                  left: Box::new(top_masked),
                  right: Box::new(bot_masked),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              }
          } else {
              // BFM: (dst & ~tmask) | (bot & tmask)
              let dst = Expr::ReadReg { reg: decoded.rd, width: decoded.width };
              let dst_masked = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(dst),
                  right: Box::new(Expr::Const { value: (!tmask) as u128 & (if decoded.width == BitWidth::W64 { 
u64::MAX as u128 } else { 0xFFFFFFFFu128 }), ty: decoded.width }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              let bot_masked = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(bot),
                  right: Box::new(Expr::Const { value: tmask as u128, ty: decoded.width }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              Expr::Binary {
                  op: BinOp::Or,
                  left: Box::new(dst_masked),
                  right: Box::new(bot_masked),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              }
          };
  
          let semantics = vec![Stmt::WriteReg {
              reg: decoded.rd,
              value: result_expr,
          }];
  
          return Instruction {
              id: InstructionId {
                  mnemonic: decoded.mnemonic.clone(),
                  encoding: Some(format!("{} (bitfield) {}", decoded.mnemonic, decoded.width.bits())),
              },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance {
                  source_path: "arm64_md_clean/bfm.md".to_string(),
                  source_sha256: "test-source-sha256".to_string(),
                  clean_sha256: "test-clean-sha256".to_string(),
              },
          };
      }
      if decoded.mnemonic == "MOVZ" || decoded.mnemonic == "MOVN" || decoded.mnemonic == "MOVK" {
          let mask: u128 = if decoded.width == BitWidth::W64 {
              u64::MAX as u128
          } else {
              0xFFFFFFFFu128
          };
          let shift = decoded.shift_amount as u32;
          let imm_val = (decoded.imm as u128) << shift;
          let result_expr = if decoded.mnemonic == "MOVZ" {
              Expr::Const {
                  value: imm_val & mask,
                  ty: decoded.width,
              }
          } else if decoded.mnemonic == "MOVN" {
              Expr::Const {
                  value: (!imm_val) & mask,
                  ty: decoded.width,
              }
          } else {
              // MOVK
              let old = Expr::ReadReg {
                  reg: decoded.rd,
                  width: decoded.width,
              };
              let clear_mask = !(0xFFFFu128 << shift) & mask;
              let cleared = Expr::Binary {
                  op: BinOp::And,
                  left: Box::new(old),
                  right: Box::new(Expr::Const {
                      value: clear_mask,
                      ty: decoded.width,
                  }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              };
              Expr::Binary {
                  op: BinOp::Or,
                  left: Box::new(cleared),
                  right: Box::new(Expr::Const {
                      value: imm_val & mask,
                      ty: decoded.width,
                  }),
                  ty: ValueType::Bits(Width::Fixed(decoded.width)),
              }
          };
  
          let semantics = vec![Stmt::WriteReg {
              reg: decoded.rd,
              value: result_expr,
          }];
  
          return Instruction {
              id: InstructionId {
                  mnemonic: decoded.mnemonic.clone(),
                  encoding: Some(format!(
                      "{} (move wide) {}",
                      decoded.mnemonic, decoded.width.bits()
                  )),
              },
              operands: Vec::new(),
              required_features: Vec::new(),
              decode: None,
              semantics,
              provenance: Provenance {
                  source_path: "arm64_md_clean/move_wide.md".to_string(),
                  source_sha256: "test-source-sha256".to_string(),
                  clean_sha256: "test-clean-sha256".to_string(),
              },
          };
      }
      let ty = ValueType::Bits(Width::Fixed(decoded.width));
  
      let rn_expr = Expr::ReadReg {
          reg: decoded.rn,
          width: decoded.width,
      };
  
      let rm_expr = Expr::ReadReg {
          reg: decoded.rm,
          width: decoded.width,
      };
  
      let shift_amount = Expr::Const {
          value: decoded.shift_amount as u128,
          ty: decoded.width,
      };
  
      let shifted = Expr::Shift {
          kind: decoded.shift,
          value: Box::new(rm_expr),
          amount: Box::new(shift_amount),
          ty: decoded.width,
      };
  
      let is_not = matches!(decoded.mnemonic.as_str(), "BIC" | "BICS" | "ORN" | "EON");
      let shifted = if is_not {
          Expr::Unary {
              op: UnaryOp::Not,
              value: Box::new(shifted),
              ty: ValueType::Bits(Width::Fixed(decoded.width)),
          }
      } else {
          shifted
      };
  
      let bin_op = match decoded.mnemonic.as_str() {
          "SUB" => BinOp::Sub,
          "AND" | "ANDS" | "BIC" | "BICS" => BinOp::And,
          "ORR" | "ORN" => BinOp::Or,
          "EOR" | "EON" => BinOp::Xor,
          _ => BinOp::Add,
      };
  
      let result = Expr::Binary {
          op: bin_op,
          left: Box::new(rn_expr),
          right: Box::new(shifted),
          ty,
      };
  
      let mut semantics = vec![Stmt::WriteReg {
          reg: decoded.rd,
          value: result.clone(),
      }];
  
      if decoded.mnemonic == "ANDS" || decoded.mnemonic == "BICS" {
          let bits = decoded.width.bits() as u128;
          let rd_read = Expr::ReadReg {
              reg: decoded.rd,
              width: decoded.width,
          };
          let n_shift = Expr::Shift {
              kind: ShiftKind::Lsr,
              value: Box::new(rd_read.clone()),
              amount: Box::new(Expr::Const {
                  value: bits - 1,
                  ty: decoded.width,
              }),
              ty: decoded.width,
          };
          let n_bool = Expr::Binary {
              op: BinOp::Eq,
              left: Box::new(n_shift),
              right: Box::new(Expr::Const {
                  value: 1,
                  ty: decoded.width,
              }),
              ty: ValueType::Bool,
          };
          semantics.push(Stmt::WriteFlag {
              flag: Flag::N,
              value: n_bool,
          });
          let z_expr = Expr::Binary {
              op: BinOp::Eq,
              left: Box::new(rd_read),
              right: Box::new(Expr::Const {
                  value: 0,
                  ty: decoded.width,
              }),
              ty: ValueType::Bool,
          };
          semantics.push(Stmt::WriteFlag {
              flag: Flag::Z,
              value: z_expr,
          });
          semantics.push(Stmt::WriteFlag {
              flag: Flag::C,
              value: Expr::Bool(false),
          });
          semantics.push(Stmt::WriteFlag {
              flag: Flag::V,
              value: Expr::Bool(false),
          });
      }
  
      Instruction {
          id: InstructionId {
              mnemonic: decoded.mnemonic.clone(),
              encoding: Some(format!(
                  "{} (shifted register) {}",
                  decoded.mnemonic, decoded.width.bits()
              )),
          },
          operands: Vec::new(),
          required_features: Vec::new(),
          decode: None,
          semantics,
          provenance: Provenance {
              source_path: "arm64_md_clean/logical_shifted.md".to_string(),
              source_sha256: "test-source-sha256".to_string(),
              clean_sha256: "test-clean-sha256".to_string(),
          },
      }
  }