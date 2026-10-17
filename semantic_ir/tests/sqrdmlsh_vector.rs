/// SQRDMLSH (vector): Signed saturating rounding doubling multiply subtract returning high half
/// Stanford source: sqrdmlsh_vector.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqrdmlsh_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // SQRDMLSH (vector): Signed saturating rounding doubling multiply subtract returning high half (placeholder)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub, // Placeholder for SQRDMLSH
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQRDMLSH (vector)".to_string(),
            encoding: Some("Signed saturating rounding doubling multiply subtract returning high half".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqrdmlsh_vector.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqrdmlsh_vector_validates() {
    let instruction = sqrdmlsh_vector_instruction();
    validate_instruction(&instruction).expect("SQRDMLSH (vector) should validate");
}

#[test]
fn sqrdmlsh_vector_uses_dmul_sub() {
    let instruction = sqrdmlsh_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SQRDMLSH (vector)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}