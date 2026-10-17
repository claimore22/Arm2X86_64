/// INCD, INCH, INCW (vector) - Increment vector by multiple of predicate constraint element count
/// Stanford source: arm64_md_clean/incd_z_zs.md
///
/// Stanford pseudocode:
///   let esize : integer = 32;  // varies: 64 for INCD, 16 for INCH, 32 for INCW
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
///       result[e*:esize] = operand1[e*:esize] + (count * imm);
///   end;
///   Z[VL](dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, ValueType, Width,
    validate_instruction,
};

fn create_inc_vector_instruction(mnemonic: &str, esize: u16, esize_bitwidth: BitWidth) -> Instruction {
    let vl = BitWidth::W256; // Example VL

    // Pattern (pat) - predicate constraint
    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 }; // Example: VL1 pattern

    // Immediate (imm4 + 1)
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 }; // imm = 1

    // Vector destination register (Zdn)
    let dn = 0u8; // Z0
    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };

    // Compute count = DecodePredCount(pat, esize)
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: esize as u128, ty: BitWidth::W8 }), // esize
        ],
        vl,
        esize: None,
        imm: None,
    };

    // count * imm (scalar, 64-bit)
    let count_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Broadcast scalar to vector using SVESpecial::Dup
    // The scalar is treated as having the element size (esize)
    let broadcast_scalar = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(count_times_imm)],
        vl,
        esize: Some(esize_bitwidth),
        imm: None,
    };

    // Element-wise addition: operand1 + broadcast_scalar
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
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
            encoding: Some("Increment vector by multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "incd_z_zs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// INCD: esize = 64 (doubleword)
fn incd_vector_sve_instruction() -> Instruction {
    create_inc_vector_instruction("INCD", 64, BitWidth::W64)
}

#[test]
fn incd_vector_sve_validates() {
    let instruction = incd_vector_sve_instruction();
    validate_instruction(&instruction).expect("INCD (vector) SVE should validate");
}

#[test]
fn incd_vector_sve_uses_esize_64() {
    let instruction = incd_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W64);
}

// INCH: esize = 16 (halfword)
fn inch_vector_sve_instruction() -> Instruction {
    create_inc_vector_instruction("INCH", 16, BitWidth::W16)
}

#[test]
fn inch_vector_sve_validates() {
    let instruction = inch_vector_sve_instruction();
    validate_instruction(&instruction).expect("INCH (vector) SVE should validate");
}

#[test]
fn inch_vector_sve_uses_esize_16() {
    let instruction = inch_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W16);
}

// INCW: esize = 32 (word)
fn incw_vector_sve_instruction() -> Instruction {
    create_inc_vector_instruction("INCW", 32, BitWidth::W32)
}

#[test]
fn incw_vector_sve_validates() {
    let instruction = incw_vector_sve_instruction();
    validate_instruction(&instruction).expect("INCW (vector) SVE should validate");
}

#[test]
fn incw_vector_sve_uses_esize_32() {
    let instruction = incw_vector_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    assert_eq!(*esize, BitWidth::W32);
}