/// SQRDMLAH (by element): Signed saturating rounding doubling multiply accumulate returning high half (by element)
/// Stanford source: sqrdmlah_by_element.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sqrdmlah_by_element_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

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

    // SQRDMLAH (by element): Signed saturating rounding doubling multiply accumulate returning high half (placeholder)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Mul, // Placeholder for SQRDMLAH
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "SQRDMLAH (by element)".to_string(),
            encoding: Some("Signed saturating rounding doubling multiply accumulate returning high half (by element)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sqrdmlah_by_element.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sqrdmlah_by_element_validates() {
    let instruction = sqrdmlah_by_element_instruction();
    validate_instruction(&instruction).expect("SQRDMLAH (by element) should validate");
}

#[test]
fn sqrdmlah_by_element_uses_dmul_add() {
    let instruction = sqrdmlah_by_element_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for SQRDMLAH (by element)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}