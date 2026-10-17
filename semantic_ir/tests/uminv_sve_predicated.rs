/// UMINV (SVE predicated): Unsigned minimum across vector (SVE predicated)
/// Stanford source: uminv_sve_predicated.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uminv_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W128;
    let pl = BitWidth::W16; // vl/8 = 16 bits for 128-bit VL
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // UMINV (SVE predicated): Unsigned minimum across vector
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::UMinV,
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
            mnemonic: "UMINV (SVE predicated)".to_string(),
            encoding: Some("Unsigned minimum across vector (SVE predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uminv_sve_predicated.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uminv_sve_predicated_validates() {
    let instruction = uminv_sve_predicated_instruction();
    validate_instruction(&instruction).expect("UMINV (SVE predicated) should validate");
}

#[test]
fn uminv_sve_predicated_uses_scalable_reduce() {
    let instruction = uminv_sve_predicated_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalableVectorReduce { op, .. } = value else {
        panic!("expected ScalableVectorReduce for UMINV (SVE predicated)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorReduceOp::UMinV);
}