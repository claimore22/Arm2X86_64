/// DECP (scalar) - Decrement scalar by count of true predicate elements
/// Stanford source: arm64_md_clean/decp_r_p_r.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);
///   let m : integer = UInt(Pm);
///   let dn : integer = UInt(Rdn);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let operand1 : bits(64) = X[dn];
///   let operand2 : bits(PL) = P[m];
///   var count : integer = 0;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement{PL}(operand2, e, esize) then
///           count = count + 1;
///       end;
///   end;
///   X[64](dn) = operand1 - count;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn decp_scalar_sve_instruction() -> Instruction {
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

    // result = operand1 - count
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(operand1),
        right: Box::new(count),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(dn),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DECP".to_string(),
            encoding: Some("Decrement scalar by count of true predicate elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "decp_r_p_r.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn decp_scalar_sve_validates() {
    let instruction = decp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("DECP (scalar) SVE should validate");
}

#[test]
fn decp_scalar_sve_uses_count_pred() {
    let instruction = decp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op: semantic_ir::BinOp::Sub, left, right, .. } = value else {
        panic!("expected Binary Sub for result");
    };

    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }));

    let Expr::SVESpecial { op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for CountPred");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::CountPred);
    assert!(matches!(operands[0].as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
}

// DECP (vector) - Decrement vector by count of true predicate elements
/// Stanford source: arm64_md_clean/decp_z_p_z.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size); // varies: 16, 32, 64
///   let m : integer = UInt(Pm);
///   let dn : integer = UInt(Zdn);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let operand1 : bits(VL) = Z[dn];
///   let operand2 : bits(PL) = P[m];
///   var count : integer = 0;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement{PL}(operand2, e, esize) then
///           count = count + 1;
///       end;
///   end;
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       result[e*:esize] = operand1[e*:esize] - count;
///   end;
///   Z[VL](dn) = result;

fn create_decp_vector_instruction(esize_bitwidth: BitWidth) -> Instruction {
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

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,
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

    Instruction {
        id: InstructionId {
            mnemonic: format!("DECP_{}", esize_name(esize_bitwidth)),
            encoding: Some("Decrement vector by count of true predicate elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "decp_z_p_z.md".to_string(),
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

// DECP vector variants
fn decp_vector_half_sve_instruction() -> Instruction {
    create_decp_vector_instruction(BitWidth::W16)
}

fn decp_vector_word_sve_instruction() -> Instruction {
    create_decp_vector_instruction(BitWidth::W32)
}

fn decp_vector_double_sve_instruction() -> Instruction {
    create_decp_vector_instruction(BitWidth::W64)
}

#[test]
fn decp_vector_half_sve_validates() {
    let instruction = decp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("DECP (vector, half) SVE should validate");
}

#[test]
fn decp_vector_half_sve_uses_esize_16() {
    let instruction = decp_vector_half_sve_instruction();

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
fn decp_vector_word_sve_validates() {
    let instruction = decp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("DECP (vector, word) SVE should validate");
}

#[test]
fn decp_vector_word_sve_uses_esize_32() {
    let instruction = decp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn decp_vector_double_sve_validates() {
    let instruction = decp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("DECP (vector, double) SVE should validate");
}

#[test]
fn decp_vector_double_sve_uses_esize_64() {
    let instruction = decp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    assert_eq!(*esize, BitWidth::W64);
}