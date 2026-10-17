/// INCB, INCD, INCH, INCW (scalar) - Increment scalar by multiple of predicate constraint element count
/// Stanford source: arm64_md_clean/incb_r_rs.md
///
/// Stanford pseudocode:
///   let esize : integer = 32;
///   let dn : integer = UInt(Rdn);
///   let pat : bits(5) = pattern;
///   let imm : integer = UInt(imm4) + 1;
///   CheckSVEEnabled();
///   let count : integer = DecodePredCount(pat, esize);
///   let VL : integer = CurrentVL();
///   let operand1 : bits(64) = X[dn];
///   X[64](dn) = operand1 + (count * imm);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn incb_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256; // Example VL
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32

    // Pattern (pat) - predicate constraint (e.g., VL8 = 8 active elements per 64-bit)
    // For INCB, esize = 8 (byte elements)
    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 }; // Example: VL1 pattern

    // Immediate (imm4 + 1)
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 }; // imm4 = 0, so imm = 1

    // Scalar destination register (Xdn)
    let dn = 0u8; // X0
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    // Compute count = DecodePredCount(pat, esize)
    // For INCB, esize = 8 (byte elements)
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: 8, ty: BitWidth::W8 }), // esize = 8
        ],
        vl,
        esize: None,
        imm: None,
    };

    // result = operand1 + (count * imm)
    // First compute count * imm
    let count_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
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
            mnemonic: "INCB".to_string(),
            encoding: Some("Increment scalar by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn incb_scalar_sve_validates() {
    let instruction = incb_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INCB (scalar) SVE should validate");
}

#[test]
fn incb_scalar_sve_uses_decode_pred_count() {
    let instruction = incb_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // The result should be operand1 + (count * imm)
    // where count comes from SVESpecial::DecodePredCount
    let Expr::Binary { op: semantic_ir::BinOp::Add, left, right, .. } = value else {
        panic!("expected Binary Add for result");
    };

    // Left should be the original operand (ReadReg X0)
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }));

    // Right should be count * imm (a Binary Mul)
    let Expr::Binary { op: semantic_ir::BinOp::Mul, left: count_expr, right: imm_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul for count * imm");
    };

    // imm_expr should be the immediate value
    assert!(matches!(imm_expr.as_ref(), Expr::Const { value: 1, ty: BitWidth::W64 }));

    // count_expr should be SVESpecial::DecodePredCount
    let Expr::SVESpecial { op, operands, .. } = count_expr.as_ref() else {
        panic!("expected SVESpecial for DecodePredCount");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::DecodePredCount);
    assert_eq!(operands.len(), 2);
    // First operand: pattern (5-bit)
    assert!(matches!(operands[0].as_ref(), Expr::Const { value: 0b00001, ty: BitWidth::W8, .. }));
    // Second operand: esize = 8
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 8, ty: BitWidth::W8 }));
}

#[test]
fn incb_scalar_sve_destination_register() {
    let instruction = incb_scalar_sve_instruction();

    let Stmt::WriteReg { reg, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    assert_eq!(*reg, Reg::X(0));
}

// Test for INCD (doubleword, esize=64)
fn incd_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 }; // VL1 pattern
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 }; // imm = 1
    let dn = 0u8;
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    // For INCD, esize = 64 (doubleword elements)
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: 64, ty: BitWidth::W8 }), // esize = 64
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

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
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
            mnemonic: "INCD".to_string(),
            encoding: Some("Increment scalar by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn incd_scalar_sve_validates() {
    let instruction = incd_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INCD (scalar) SVE should validate");
}

#[test]
fn incd_scalar_sve_uses_esize_64() {
    let instruction = incd_scalar_sve_instruction();

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

    // Second operand should be esize = 64
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 64, ty: BitWidth::W8 }));
}

// Test for INCH (halfword, esize=16)
fn inch_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 };
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 };
    let dn = 0u8;
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: 16, ty: BitWidth::W8 }), // esize = 16
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

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
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
            mnemonic: "INCH".to_string(),
            encoding: Some("Increment scalar by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn inch_scalar_sve_validates() {
    let instruction = inch_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INCH (scalar) SVE should validate");
}

#[test]
fn inch_scalar_sve_uses_esize_16() {
    let instruction = inch_scalar_sve_instruction();

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

    // Second operand should be esize = 16
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 16, ty: BitWidth::W8 }));
}

// Test for INCW (word, esize=32)
fn incw_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 };
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 };
    let dn = 0u8;
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: 32, ty: BitWidth::W8 }), // esize = 32
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

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
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
            mnemonic: "INCW".to_string(),
            encoding: Some("Increment scalar by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn incw_scalar_sve_validates() {
    let instruction = incw_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INCW (scalar) SVE should validate");
}

#[test]
fn incw_scalar_sve_uses_esize_32() {
    let instruction = incw_scalar_sve_instruction();

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

    // Second operand should be esize = 32
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 32, ty: BitWidth::W8 }));
}