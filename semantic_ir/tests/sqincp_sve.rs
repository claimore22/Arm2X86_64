/// SQINCP, UQINCP (scalar) - Signed/Unsigned saturating increment scalar by count of true predicate elements
/// Stanford source: arm64_md_clean/sqincp_r_p_r.md, uqincp_r_p_r.md
///
/// Stanford pseudocode (SQINCP):
///   let unsigned : boolean = FALSE;
///   let ssize : integer = 64;
///   ...
///   (result, -) = SignedSatQ{ssize}(SInt(operand1) + count);
///   X[64](dn) = SignExtend{64}(result);
///
/// Stanford pseudocode (UQINCP):
///   let unsigned : boolean = TRUE;
///   let ssize : integer = 64;
///   ...
///   (result, -) = UnsignedSatQ{ssize}(UInt(operand1) + count);
///   X[64](dn) = ZeroExtend{64}(result);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn create_sqincp_scalar_instruction(unsigned: bool) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let dn = 0u8;
    let m = 0u8;

    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };
    let predicate = Expr::ReadPredReg { reg: m, pl };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: None,
        operands: vec![Box::new(predicate)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    // Use saturating add operation
    let bin_op = if unsigned {
        semantic_ir::BinOp::UQAdd
    } else {
        semantic_ir::BinOp::SQAdd
    };

    let result = Expr::Binary {
        op: bin_op,
        left: Box::new(operand1),
        right: Box::new(count),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(dn),
        value: result,
    }];

    let mnemonic = if unsigned { "UQINCP" } else { "SQINCP" };

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("{} saturating increment scalar by count of true predicate elements", mnemonic)),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if unsigned { "uqincp_r_p_r.md" } else { "sqincp_r_p_r.md" }.to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// SQINCP (scalar) - signed saturating
fn sqincp_scalar_sve_instruction() -> Instruction {
    create_sqincp_scalar_instruction(false)
}

// UQINCP (scalar) - unsigned saturating
fn uqincp_scalar_sve_instruction() -> Instruction {
    create_sqincp_scalar_instruction(true)
}

#[test]
fn sqincp_scalar_sve_validates() {
    let instruction = sqincp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("SQINCP (scalar) SVE should validate");
}

#[test]
fn uqincp_scalar_sve_validates() {
    let instruction = uqincp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("UQINCP (scalar) SVE should validate");
}

#[test]
fn sqincp_scalar_sve_uses_sqadd() {
    let instruction = sqincp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, .. } = value else {
        panic!("expected Binary");
    };

    assert_eq!(*op, semantic_ir::BinOp::SQAdd);
}

#[test]
fn uqincp_scalar_sve_uses_uqadd() {
    let instruction = uqincp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, .. } = value else {
        panic!("expected Binary");
    };

    assert_eq!(*op, semantic_ir::BinOp::UQAdd);
}

// SQINCP, UQINCP (vector) - Signed/Unsigned saturating increment vector by count of true predicate elements
/// Stanford source: arm64_md_clean/sqincp_z_p_z.md, uqincp_z_p_z.md
///
/// Stanford pseudocode (SQINCP vector):
///   let unsigned : boolean = FALSE;
///   ...
///   for e = 0 to elements-1 do
///       let opelt : bits(esize) = operand1[e*:esize];
///       (result[e*:esize], -) = SignedSatQ{esize}(SInt(opelt) + count);
///   end;
///   Z[VL](dn) = result;
///
/// Stanford pseudocode (UQINCP vector):
///   let unsigned : boolean = TRUE;
///   ...
///   for e = 0 to elements-1 do
///       let opelt : bits(esize) = operand1[e*:esize];
///       (result[e*:esize], -) = UnsignedSatQ{esize}(UInt(opelt) + count);
///   end;
///   Z[VL](dn) = result;

fn create_sqincp_vector_instruction(unsigned: bool, esize_bitwidth: BitWidth) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let m = 0u8;
    let predicate = Expr::ReadPredReg { reg: m, pl };

    let dn = 0u8;
    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: None,
        operands: vec![Box::new(predicate)],
        vl,
        esize: Some(BitWidth::W8),
        imm: None,
    };

    // Broadcast scalar count to vector
    let broadcast_count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(count)],
        vl,
        esize: Some(esize_bitwidth),
        imm: None,
    };

    // Use saturating vector add operation
    let bin_op = if unsigned {
        semantic_ir::primitives::SVEVectorBinOp::UQAdd
    } else {
        semantic_ir::primitives::SVEVectorBinOp::SQAdd
    };

    let result = Expr::ScalableVectorMap2 {
        op: bin_op,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand1.clone()),
        right: Box::new(broadcast_count),
        destination: Box::new(operand1),
        vl,
        esize: esize_bitwidth,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: dn,
        value: result,
        vl,
    }];

    let mnemonic = if unsigned { "UQINCP" } else { "SQINCP" };

    Instruction {
        id: InstructionId {
            mnemonic: format!("{}_{}", mnemonic, esize_name(esize_bitwidth)),
            encoding: Some(format!("{} saturating increment vector by count of true predicate elements", mnemonic)),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if unsigned { "uqincp_z_p_z.md" } else { "sqincp_z_p_z.md" }.to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn esize_name(esize: BitWidth) -> &'static str {
    match esize {
        BitWidth::W16 => "H",
        BitWidth::W32 => "W",
        BitWidth::W64 => "D",
        _ => "UNKNOWN",
    }
}

// SQINCP vector variants
fn sqincp_vector_half_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(false, BitWidth::W16)
}

fn sqincp_vector_word_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(false, BitWidth::W32)
}

fn sqincp_vector_double_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(false, BitWidth::W64)
}

// UQINCP vector variants
fn uqincp_vector_half_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(true, BitWidth::W16)
}

fn uqincp_vector_word_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(true, BitWidth::W32)
}

fn uqincp_vector_double_sve_instruction() -> Instruction {
    create_sqincp_vector_instruction(true, BitWidth::W64)
}

#[test]
fn sqincp_vector_half_sve_validates() {
    let instruction = sqincp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("SQINCP (vector, half) SVE should validate");
}

#[test]
fn sqincp_vector_half_sve_uses_esize_16() {
    let instruction = sqincp_vector_half_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQAdd);
    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn sqincp_vector_word_sve_validates() {
    let instruction = sqincp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("SQINCP (vector, word) SVE should validate");
}

#[test]
fn sqincp_vector_word_sve_uses_esize_32() {
    let instruction = sqincp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQAdd);
    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn sqincp_vector_double_sve_validates() {
    let instruction = sqincp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("SQINCP (vector, double) SVE should validate");
}

#[test]
fn sqincp_vector_double_sve_uses_esize_64() {
    let instruction = sqincp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQAdd);
    assert_eq!(*esize, BitWidth::W64);
}

#[test]
fn uqincp_vector_half_sve_validates() {
    let instruction = uqincp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("UQINCP (vector, half) SVE should validate");
}

#[test]
fn uqincp_vector_half_sve_uses_esize_16() {
    let instruction = uqincp_vector_half_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQAdd);
    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn uqincp_vector_word_sve_validates() {
    let instruction = uqincp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("UQINCP (vector, word) SVE should validate");
}

#[test]
fn uqincp_vector_word_sve_uses_esize_32() {
    let instruction = uqincp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQAdd);
    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn uqincp_vector_double_sve_validates() {
    let instruction = uqincp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("UQINCP (vector, double) SVE should validate");
}

#[test]
fn uqincp_vector_double_sve_uses_esize_64() {
    let instruction = uqincp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQAdd);
    assert_eq!(*esize, BitWidth::W64);
}