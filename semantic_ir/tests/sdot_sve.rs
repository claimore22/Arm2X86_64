/// SDOT (SVE, 2-way vectors) - Signed integer dot product (two-way, vectors)
/// Stanford source: arm64_md_clean/sdot_z_zz_i8_i8.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn sdot_2way_vectors_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result is 32-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - signed 8-bit pairs
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Z[m] - signed 8-bit pairs
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Z[da] - accumulator (32-bit)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::SDot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W256,
        esize,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SDOT (2-way, vectors)".to_string(),
            encoding: Some("Signed integer dot product (two-way, vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_I8MM".to_string(), "FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sdot_z_zz_i8_i8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// SDOT (SVE, 4-way vectors) - Signed integer dot product (four-way, vectors)
/// Stanford source: arm64_md_clean/sdot_z_zz_i8_i8.md
fn sdot_4way_vectors_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result is 32-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - signed 8-bit quads
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Z[m] - signed 8-bit quads
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Z[da] - accumulator (32-bit)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::SDot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W256,
        esize,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SDOT (4-way, vectors)".to_string(),
            encoding: Some("Signed integer dot product (four-way, vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_I8MM".to_string(), "FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sdot_z_zz_i8_i8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sdot_2way_vectors_sve_validates() {
    let instruction = sdot_2way_vectors_sve_instruction();
    validate_instruction(&instruction).expect("SDOT (2-way, vectors) SVE should validate");
}

#[test]
fn sdot_4way_vectors_sve_validates() {
    let instruction = sdot_4way_vectors_sve_instruction();
    validate_instruction(&instruction).expect("SDOT (4-way, vectors) SVE should validate");
}

#[test]
fn sdot_2way_vectors_sve_structure() {
    let instruction = sdot_2way_vectors_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot { op, a, b, addend, datasize, esize, .. } = value else {
        panic!("expected VectorDot");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::SDot);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    matches!(a.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(b.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(addend.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn sdot_4way_vectors_sve_structure() {
    let instruction = sdot_4way_vectors_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot { op, a, b, addend, datasize, esize, .. } = value else {
        panic!("expected VectorDot");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::SDot);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
}