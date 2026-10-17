/// FCADD: Floating-point complex add
/// Stanford source: fcadd_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fcadd_instruction() -> Instruction {
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

    // FCADD: Floating-point complex add
    // Complex numbers are represented as pairs of FP values
    // Result: (a.real + b.real, a.imag - b.imag) or similar
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for complex add
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
            mnemonic: "FCADD".to_string(),
            encoding: Some("Floating-point complex add".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcadd_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcadd_validates() {
    let instruction = fcadd_instruction();
    validate_instruction(&instruction).expect("FCADD should validate");
}

#[test]
fn fcadd_uses_vector_map2() {
    let instruction = fcadd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FCADD");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}