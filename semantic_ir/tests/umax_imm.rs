/// UMAX (immediate): Unsigned maximum (immediate)
/// Stanford source: umax_imm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn umax_imm_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::Const {
        value: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128, // Example immediate
        ty: BitWidth::W128,
    };

    // UMAX (immediate): Unsigned maximum with immediate
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::UMax,
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
            mnemonic: "UMAX (immediate)".to_string(),
            encoding: Some("Unsigned maximum (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "umax_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn umax_imm_validates() {
    let instruction = umax_imm_instruction();
    validate_instruction(&instruction).expect("UMAX (immediate) should validate");
}

#[test]
fn umax_imm_uses_vector_map2() {
    let instruction = umax_imm_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for UMAX (immediate)");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::UMax);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::Const { ty: BitWidth::W128, .. }));
}
