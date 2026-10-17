/// INCP (scalar) - Increment scalar by count of true predicate elements
/// Stanford source: arm64_md_clean/incp_r_p_r.md
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
///   X[64](dn) = operand1 + count;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn incp_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256; // Example VL
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32
    let esize = BitWidth::W8; // Default esize for predicate (8-bit elements)

    // Scalar destination register (Xdn)
    let dn = 0u8; // X0
    let operand1 = Expr::ReadReg { reg: Reg::X(dn), width: BitWidth::W64 };

    // Source predicate register (Pm)
    let m = 0u8; // P0
    let predicate = Expr::ReadPredReg { reg: m, pl };

    // Count true predicate elements using SVESpecial::CountPred
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: None,
        operands: vec![Box::new(predicate)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    // result = operand1 + count
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
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
            mnemonic: "INCP".to_string(),
            encoding: Some("Increment scalar by count of true predicate elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incp_r_p_r.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn incp_scalar_sve_validates() {
    let instruction = incp_scalar_sve_instruction();
    validate_instruction(&instruction).expect("INCP (scalar) SVE should validate");
}

#[test]
fn incp_scalar_sve_uses_count_pred() {
    let instruction = incp_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // The result should be operand1 + count
    let Expr::Binary { op: semantic_ir::BinOp::Add, left, right, .. } = value else {
        panic!("expected Binary Add for result");
    };

    // Left should be the original operand (ReadReg X0)
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 }));

    // Right should be SVESpecial::CountPred
    let Expr::SVESpecial { op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial for CountPred");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::CountPred);
    assert_eq!(operands.len(), 1);
    // Operand should be the predicate register
    assert!(matches!(operands[0].as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
}

#[test]
fn incp_scalar_sve_destination_register() {
    let instruction = incp_scalar_sve_instruction();

    let Stmt::WriteReg { reg, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    assert_eq!(*reg, Reg::X(0));
}

// INCP (vector) - Increment vector by count of true predicate elements
/// Stanford source: arm64_md_clean/incp_z_p_z.md
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
///       result[e*:esize] = operand1[e*:esize] + count;
///   end;
///   Z[VL](dn) = result;
fn create_incp_vector_instruction(mnemonic: &str, esize: u16, esize_bitwidth: BitWidth) -> Instruction {
    let vl = BitWidth::W256; // Example VL
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32

    // Source predicate register (Pm)
    let m = 0u8; // P0
    let predicate = Expr::ReadPredReg { reg: m, pl };

    // Vector destination register (Zdn)
    let dn = 0u8; // Z0
    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };

    // Count true predicate elements using SVESpecial::CountPred
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: None,
        operands: vec![Box::new(predicate)],
        vl,
        esize: Some(BitWidth::W8), // Predicate elements are 8-bit
        imm: None,
    };

    // Broadcast scalar count to vector using SVESpecial::Dup
    let broadcast_count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(count)],
        vl,
        esize: Some(esize_bitwidth),
        imm: None,
    };

    // Element-wise addition: operand1 + broadcast_count
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
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
            mnemonic: mnemonic.to_string(),
            encoding: Some("Increment vector by count of true predicate elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incp_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// INCP (vector) with esize = 16 (halfword)
fn incp_vector_half_sve_instruction() -> Instruction {
    create_incp_vector_instruction("INCP_H", 16, BitWidth::W16)
}

// INCP (vector) with esize = 32 (word)
fn incp_vector_word_sve_instruction() -> Instruction {
    create_incp_vector_instruction("INCP_W", 32, BitWidth::W32)
}

// INCP (vector) with esize = 64 (doubleword)
fn incp_vector_double_sve_instruction() -> Instruction {
    create_incp_vector_instruction("INCP_D", 64, BitWidth::W64)
}

#[test]
fn incp_vector_half_sve_validates() {
    let instruction = incp_vector_half_sve_instruction();
    validate_instruction(&instruction).expect("INCP (vector, half) SVE should validate");
}

#[test]
fn incp_vector_half_sve_uses_esize_16() {
    let instruction = incp_vector_half_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W16);
}

#[test]
fn incp_vector_word_sve_validates() {
    let instruction = incp_vector_word_sve_instruction();
    validate_instruction(&instruction).expect("INCP (vector, word) SVE should validate");
}

#[test]
fn incp_vector_word_sve_uses_esize_32() {
    let instruction = incp_vector_word_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W32);
}

#[test]
fn incp_vector_double_sve_validates() {
    let instruction = incp_vector_double_sve_instruction();
    validate_instruction(&instruction).expect("INCP (vector, double) SVE should validate");
}

#[test]
fn incp_vector_double_sve_uses_esize_64() {
    let instruction = incp_vector_double_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W64);
}