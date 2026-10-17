/// FNMSUB - fnmsub_float.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn fnmsub_instruction() -> Instruction {
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

    // FNMSUB: floating-point negated multiply-subtract
    // Use FPAdd as placeholder
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd,
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
            mnemonic: "FNMSUB".to_string(),
            encoding: Some("FNMSUB".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fnmsub_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fnmsub_validates() {
    let instruction = fnmsub_instruction();
    validate_instruction(&instruction).expect("FNMSUB should validate");
}