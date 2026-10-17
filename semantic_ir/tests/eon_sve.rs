/// EON (SVE) - Bitwise exclusive-OR with inverted immediate (unpredicated)
/// Stanford source: arm64_md_clean/eon_eor_z_zi.md
///
/// Stanford pseudocode:
///   The description of EOR (immediate) gives the operational pseudocode for this instruction.
///   EON is a pseudo-instruction of EOR (immediate): EON Zdn, imm = EOR Zdn, NOT(imm)
///   imm is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn eon_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let dn = 0u8; // Zdn

    // EON is EOR with inverted immediate: EON Zdn, imm = EOR Zdn, NOT(imm)
    // The immediate is a 64-bit bitmask with a single run of ones/zeros
    // NOT(imm) = imm ^ 0xFFFFFFFFFFFFFFFF
    let imm = Expr::Const { value: 0xAAAAAAAAAAAAAAAAu128, ty: BitWidth::W64 };
    let not_imm = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(imm),
        ty: semantic_ir::types::ValueType::Bits(semantic_ir::types::Width::Fixed(BitWidth::W64)),
    };

    // Broadcast NOT(imm) to all elements using SVESpecial::Dup
    let broadcast_not_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(not_imm)],
        vl: BitWidth::W256,
        esize: Some(BitWidth::W64),
        imm: Some(0),
    };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Xor,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        right: Box::new(broadcast_not_imm),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }),
        vl: BitWidth::W256,
        esize: BitWidth::W64,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "EON".to_string(),
            encoding: Some("Bitwise exclusive-OR with inverted immediate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eon_eor_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eon_sve_validates() {
    let instruction = eon_sve_instruction();
    validate_instruction(&instruction).expect("EON SVE should validate");
}

#[test]
fn eon_sve_uses_xor_with_not() {
    let instruction = eon_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, right, .. } = value else {
        panic!("expected ScalableVectorMap2 for EON SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Xor);

    // Right operand should be a broadcast of NOT(imm) using SVESpecial::Dup
    let Expr::SVESpecial { op, operands, .. } = right.as_ref() else {
        panic!("expected SVESpecial Dup for broadcast inverted immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    
    // The operand of Dup should be a Unary Not
    let Expr::Unary { op, .. } = operands[0].as_ref() else {
        panic!("expected Unary Not for inverted immediate");
    };
    assert_eq!(*op, semantic_ir::UnaryOp::Not);
}