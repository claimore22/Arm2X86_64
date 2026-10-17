/// UMAXV (SVE): Unsigned maximum across vector (SVE)
/// Stanford source: umaxv_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn umaxv_sve_instruction() -> Instruction {
    let vl = BitWidth::W128;
    let pl = BitWidth::W16; // vl/8 = 16 bits for 128-bit VL
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // UMAXV (SVE): Unsigned maximum across vector
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::UMaxV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UMAXV (SVE)".to_string(),
            encoding: Some("Unsigned maximum across vector (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "umaxv_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn umaxv_sve_validates() {
    let instruction = umaxv_sve_instruction();
    validate_instruction(&instruction).expect("UMAXV (SVE) should validate");
}

#[test]
fn umaxv_sve_uses_scalable_reduce() {
    let instruction = umaxv_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalableVectorReduce { op, .. } = value else {
        panic!("expected ScalableVectorReduce for UMAXV (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorReduceOp::UMaxV);
}