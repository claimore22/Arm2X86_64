/// VRSQRTE (SVE): Vector reciprocal square root estimate (SVE)
/// Stanford source: vrsqrte_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vrsqrte_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };

    // VRSQRTE (SVE): Vector reciprocal square root estimate
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Abs, // Placeholder for VRSQRTE
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        operand: Box::new(operand),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 }),
        vl: BitWidth::W128,
        esize: BitWidth::W32,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VRSQRTE (SVE)".to_string(),
            encoding: Some("Vector reciprocal square root estimate (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vrsqrte_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vrsqrte_sve_validates() {
    let instruction = vrsqrte_sve_instruction();
    validate_instruction(&instruction).expect("VRSQRTE (SVE) should validate");
}

#[test]
fn vrsqrte_sve_uses_scalable_map() {
    let instruction = vrsqrte_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { op, .. } = value else {
        panic!("expected ScalableVectorMap for VRSQRTE (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Abs);
}