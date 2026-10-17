/// USDOT (vector) - usdot_advsimd_vec.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn usdot_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let a = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let b = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };
    let addend = Expr::ReadReg {
        reg: Reg::V(0),
        width: BitWidth::W128,
    };

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::SDot,
        a: Box::new(a),
        b: Box::new(b),
        addend: Box::new(addend),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::SystemReg { reg: 0 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "USDOT (vector)".to_string(),
            encoding: Some("USDOT (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "usdot_advsimd_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn usdot_vector_validates() {
    let instruction = usdot_vector_instruction();
    validate_instruction(&instruction).expect("USDOT (vector) should validate");
}
