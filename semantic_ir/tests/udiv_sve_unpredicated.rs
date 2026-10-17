/// UDIV (SVE unpredicated): Unsigned divide (SVE)
/// Stanford source: udiv_sve_unpredicated.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn udiv_sve_unpredicated_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };
    let right = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 };

    // UDIV (SVE unpredicated): Unsigned divide
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Udiv,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        left: Box::new(left),
        right: Box::new(right),
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
            mnemonic: "UDIV (SVE unpredicated)".to_string(),
            encoding: Some("Unsigned divide (SVE unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "udiv_sve_unpredicated.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn udiv_sve_unpredicated_validates() {
    let instruction = udiv_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("UDIV (SVE unpredicated) should validate");
}

#[test]
fn udiv_sve_unpredicated_uses_scalable_map2() {
    let instruction = udiv_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, .. } = value else {
        panic!("expected ScalableVectorMap2 for UDIV (SVE unpredicated)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Udiv);
}