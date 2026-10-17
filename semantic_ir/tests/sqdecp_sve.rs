/// SQDECP, UQDECP (scalar) - Signed/Unsigned saturating decrement scalar by count of true predicate elements
/// Stanford source: arm64_md_clean/sqdecp_r_p_r.md, uqdecp_r_p_r.md
///
/// Stanford pseudocode (SQDECP):
///   let unsigned : boolean = FALSE;
///   let ssize : integer = 64;
///   ...
///   (result, -) = SignedSatQ{ssize}(SInt(operand1) - count);
///   X[64](dn) = SignExtend{64}(result);
///
/// Stanford pseudocode (UQDECP):
///   let unsigned : boolean = TRUE;
///   let ssize : integer = 64;
///   ...
///   (result, -) = UnsignedSatQ{ssize}(UInt(operand1) - count);
///   X[64](dn) = ZeroExtend{64}(result);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn create_sqdecp_scalar_instruction(unsigned: bool) -> Instruction {
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

    // Use saturating subtract operation
    let bin_op = if unsigned {
        semantic_ir::BinOp::UQSub
    } else {
        semantic_ir::BinOp::SQSub
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

    let mnemonic = if unsigned { "UQDECP" } else { "SQDECP" };

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("{} saturating decrement scalar by count of true predicate elements", mnemonic)),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if unsigned { "uqdecp_r_p_r.md" } else { "sqdecp_r_p_r.md" }.to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// SQDECP (scalar) - signed saturating
fn sqdecp_scalar_sve_instruction() -> Instruction {
    create_sqdecp_scalar_instruction(false)
}

// UQDECP (scalar) - unsigned saturating
fn uqdecp_scalar_sve_instruction() -> Instruction {
    create_sqdecp_scalar_instruction(true)
}

#[test]
fn sqdecp_scalar_sve_validates() {
    let instruction = sqdecp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("SQDECP (scalar) SVE should validate");
}

#[test]
fn uqdecp_scalar_sve_validates() {
    let instruction = uqdecp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("UQDECP (scalar) SVE should validate");
}

#[test]
fn sqdecp_scalar_sve_uses_sqsub() {
    let instruction = sqdecp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, .. } = value else {
        panic!("expected Binary");
    };

    assert_eq!(*op, semantic_ir::BinOp::SQSub);
}

#[test]
fn uqdecp_scalar_sve_uses_uqsub() {
    let instruction = uqdecp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, .. } = value else {
        panic!("expected Binary");
    };

    assert_eq!(*op, semantic_ir::BinOp::UQSub);
}

// SQDECP, UQDECP (vector) - Signed/Unsigned saturating decrement vector by count of true predicate elements
/// Stanford source: arm64_md_clean/sqdecp_z_p_z.md, uqdecp_z_p_z.md
///
/// Stanford pseudocode (SQDECP vector):
///   let unsigned : boolean = FALSE;
///   ...
///   for e = 0 to elements-1 do
///       let opelt : bits(esize) = operand1[e*:esize];
///       (result[e*:esize], -) = SignedSatQ{esize}(SInt(opelt) - count);
///   end;
///   Z[VL](dn) = result;
///
/// Stanford pseudocode (UQDECP vector):
///   let unsigned : boolean = TRUE;
///   ...
///   for e = 0 to elements-1 do
///       let opelt : bits(esize) = operand1[e*:esize];
///       (result[e*:esize], -) = UnsignedSatQ{esize}(UInt(opelt) - count);
///   end;
///   Z[VL](dn) = result;

fn create_sqdecp_vector_instruction(unsigned: bool, esize_bitwidth: BitWidth) -> Instruction {
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

    let broadcast_count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(count)],
        vl,
        esize: Some(esize_bitwidth),
        imm: None,
    };

    // Use saturating vector subtract operation
    let bin_op = if unsigned {
        semantic_ir::primitives::SVEVectorBinOp::UQSub
    } else {
        semantic_ir::primitives::SVEVectorBinOp::SQSub
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

    let mnemonic = if unsigned { "UQDECP" } else { "SQDECP" };

    Instruction {
        id: InstructionId {
            mnemonic: format!("{}_{}", mnemonic, esize_name(esize_bitwidth)),
            encoding: Some(format!("{} saturating decrement vector by count of true predicate elements", mnemonic)),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if unsigned { "uqdecp_z_p_z.md" } else { "sqdecp_z_p_z.md" }.to_string(),
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

// SQDECP vector variants
fn sqdecp_vector_half_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(false, BitWidth::W16)
}

fn sqdecp_vector_word_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(false, BitWidth::W32)
}

fn sqdecp_vector_double_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(false, BitWidth::W64)
}

// UQDECP vector variants
fn uqdecp_vector_half_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(true, BitWidth::W16)
}

fn uqdecp_vector_word_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(true, BitWidth::W32)
}

fn uqdecp_vector_double_sve_instruction() -> Instruction {
    create_sqdecp_vector_instruction(true, BitWidth::W64)
}

#[test]
fn sqdecp_vector_half_sve_validates() {
    let instruction = sqdecp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("SQDECP (vector, half) SVE should validate");
}

#[test]
fn sqdecp_vector_half_sve_uses_esize_16() {
    let instruction = sqdecp_vector_half_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQSub);
    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn sqdecp_vector_word_sve_validates() {
    let instruction = sqdecp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("SQDECP (vector, word) SVE should validate");
}

#[test]
fn sqdecp_vector_word_sve_uses_esize_32() {
    let instruction = sqdecp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQSub);
    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn sqdecp_vector_double_sve_validates() {
    let instruction = sqdecp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("SQDECP (vector, double) SVE should validate");
}

#[test]
fn sqdecp_vector_double_sve_uses_esize_64() {
    let instruction = sqdecp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SQSub);
    assert_eq!(*esize, BitWidth::W64);
}

#[test]
fn uqdecp_vector_half_sve_validates() {
    let instruction = uqdecp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("UQDECP (vector, half) SVE should validate");
}

#[test]
fn uqdecp_vector_half_sve_uses_esize_16() {
    let instruction = uqdecp_vector_half_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQSub);
    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn uqdecp_vector_word_sve_validates() {
    let instruction = uqdecp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("UQDECP (vector, word) SVE should validate");
}

#[test]
fn uqdecp_vector_word_sve_uses_esize_32() {
    let instruction = uqdecp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQSub);
    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn uqdecp_vector_double_sve_validates() {
    let instruction = uqdecp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("UQDECP (vector, double) SVE should validate");
}

#[test]
fn uqdecp_vector_double_sve_uses_esize_64() {
    let instruction = uqdecp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::UQSub);
    assert_eq!(*esize, BitWidth::W64);
}