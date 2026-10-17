/// UMOV (SVE): Unsigned move vector element to general-purpose register (SVE)
/// Stanford source: umov_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn umov_sve_instruction() -> Instruction {
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
    // UMOV (SVE): Unsigned move vector element to general-purpose register
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for UMOV
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
            mnemonic: "UMOV (SVE)".to_string(),
            encoding: Some("Unsigned move vector element to general-purpose register (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "umov_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn umov_sve_validates() {
    let instruction = umov_sve_instruction();
    validate_instruction(&instruction).expect("UMOV (SVE) should validate");
}

#[test]
fn umov_sve_uses_vector_map2() {
    let instruction = umov_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for UMOV (SVE)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}
