/// DECB, DECD, DECH, DECW (scalar) - Decrement scalar by multiple of predicate constraint element count
/// Stanford source: arm64_md_clean/decb_r_rs.md
///
/// Stanford pseudocode:
///   let esize : integer = 32;  // varies: 8 for DECB, 64 for DECD, 16 for DECH, 32 for DECW
///   let dn : integer = UInt(Rdn);
///   let pat : bits(5) = pattern;
///   let imm : integer = UInt(imm4) + 1;
///   CheckSVEEnabled();
///   let count : integer = DecodePredCount(pat, esize);
///   let VL : integer = CurrentVL();
///   let operand1 : bits(64) = X[dn];
///   X[64](dn) = operand1 - (count * imm);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn create_dec_scalar_instruction(mnemonic: &str, esize: u16) -> Instruction {
    let vl = BitWidth::W256;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 };
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 };

    let dn = 0u8;
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: esize as u128, ty: BitWidth::W8 }),
        ],
        vl,
        esize: None,
        imm: None,
    };

    let count_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // result = operand1 - (count * imm)
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(operand1),
        right: Box::new(count_times_imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(dn),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Decrement scalar by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "decb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// DECB: esize = 8 (byte)
fn decb_scalar_sve_instruction() -> Instruction {
    create_dec_scalar_instruction("DECB", 8)
}

// DECD: esize = 64 (doubleword)
fn decd_scalar_sve_instruction() -> Instruction {
    create_dec_scalar_instruction("DECD", 64)
}

// DECH: esize = 16 (halfword)
fn dech_scalar_sve_instruction() -> Instruction {
    create_dec_scalar_instruction("DECH", 16)
}

// DECW: esize = 32 (word)
fn decw_scalar_sve_instruction() -> Instruction {
    create_dec_scalar_instruction("DECW", 32)
}

#[test]
fn decb_scalar_sve_validates() {
    let instruction = decb_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DECB (scalar) SVE should validate");
}

#[test]
fn decb_scalar_sve_uses_decode_pred_count() {
    let instruction = decb_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op: semantic_ir::BinOp::Sub, left, right, .. } = value else {
        panic!("expected Binary Sub for result");
    };

    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }));

    let Expr::Binary { op: semantic_ir::BinOp::Mul, left: count_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul for count * imm");
    };

    let Expr::SVESpecial { op, operands, .. } = count_expr.as_ref() else {
        panic!("expected SVESpecial for DecodePredCount");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::DecodePredCount);
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 8, ty: BitWidth::W8 }));
}

#[test]
fn decd_scalar_sve_validates() {
    let instruction = decd_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DECD (scalar) SVE should validate");
}

#[test]
fn decd_scalar_sve_uses_esize_64() {
    let instruction = decd_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { right, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::Binary { left: count_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul");
    };

    let Expr::SVESpecial { operands, .. } = count_expr.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 64, ty: BitWidth::W8 }));
}

#[test]
fn dech_scalar_sve_validates() {
    let instruction = dech_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DECH (scalar) SVE should validate");
}

#[test]
fn dech_scalar_sve_uses_esize_16() {
    let instruction = dech_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { right, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::Binary { left: count_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul");
    };

    let Expr::SVESpecial { operands, .. } = count_expr.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 16, ty: BitWidth::W8 }));
}

#[test]
fn decw_scalar_sve_validates() {
    let instruction = decw_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DECW (scalar) SVE should validate");
}

#[test]
fn decw_scalar_sve_uses_esize_32() {
    let instruction = decw_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { right, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::Binary { left: count_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul");
    };

    let Expr::SVESpecial { operands, .. } = count_expr.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 32, ty: BitWidth::W8 }));
}

// DECD, DECH, DECW (vector) - Decrement vector by multiple of predicate constraint element count
/// Stanford source: arm64_md_clean/decd_z_zs.md
///
/// Stanford pseudocode:
///   let esize : integer = 32;  // varies: 64 for DECD, 16 for DECH, 32 for DECW
///   let dn : integer = UInt(Zdn);
///   let pat : bits(5) = pattern;
///   let imm : integer = UInt(imm4) + 1;
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let count : integer = DecodePredCount(pat, esize);
///   let operand1 : bits(VL) = Z[dn];
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       result[e*:esize] = operand1[e*:esize] - (count * imm);
///   end;
///   Z[VL](dn) = result;

fn create_dec_vector_instruction(mnemonic: &str, esize: u16, esize_bitwidth: BitWidth) -> Instruction {
    let vl = BitWidth::W256;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 };
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 };

    let dn = 0u8;
    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: esize as u128, ty: BitWidth::W8 }),
        ],
        vl,
        esize: None,
        imm: None,
    };

    let count_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let broadcast_scalar = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(count_times_imm)],
        vl,
        esize: Some(esize_bitwidth),
        imm: None,
    };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand1.clone()),
        right: Box::new(broadcast_scalar),
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

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Decrement vector by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "decd_z_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// DECD vector: esize = 64
fn decd_vector_sve_instruction() -> Instruction {
    create_dec_vector_instruction("DECD", 64, BitWidth::W64)
}

// DECH vector: esize = 16
fn dech_vector_sve_instruction() -> Instruction {
    create_dec_vector_instruction("DECH", 16, BitWidth::W16)
}

// DECW vector: esize = 32
fn decw_vector_sve_instruction() -> Instruction {
    create_dec_vector_instruction("DECW", 32, BitWidth::W32)
}

#[test]
fn decd_vector_sve_validates() {
    let instruction = decd_vector_sve_instruction();
    validate_instruction(&instruction).expect("DECD (vector) SVE should validate");
}

#[test]
fn decd_vector_sve_uses_esize_64() {
    let instruction = decd_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    assert_eq!(*esize, BitWidth::W64);
}

#[test]
fn dech_vector_sve_validates() {
    let instruction = dech_vector_sve_instruction();
    validate_instruction(&instruction).expect("DECH (vector) SVE should validate");
}

#[test]
fn dech_vector_sve_uses_esize_16() {
    let instruction = dech_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn decw_vector_sve_validates() {
    let instruction = decw_vector_sve_instruction();
    validate_instruction(&instruction).expect("DECW (vector) SVE should validate");
}

#[test]
fn decw_vector_sve_uses_esize_32() {
    let instruction = decw_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    assert_eq!(*esize, BitWidth::W32);
}