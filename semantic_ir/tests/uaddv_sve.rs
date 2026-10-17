/// UADDV (SVE): Unsigned add across vector (SVE)
/// Stanford source: uaddv_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uaddv_sve_instruction() -> Instruction {
    let vl = BitWidth::W128;
    let pl = BitWidth::W16; // vl/8 = 16 bits for 128-bit VL
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // UADDV (SVE): Unsigned add across vector
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::UAddV,
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
            mnemonic: "UADDV (SVE)".to_string(),
            encoding: Some("Unsigned add across vector (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uaddv_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uaddv_sve_validates() {
    let instruction = uaddv_sve_instruction();
    validate_instruction(&instruction).expect("UADDV (SVE) should validate");
}

#[test]
fn uaddv_sve_uses_scalable_reduce() {
    let instruction = uaddv_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalableVectorReduce { op, .. } = value else {
        panic!("expected ScalableVectorReduce for UADDV (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorReduceOp::UAddV);
}