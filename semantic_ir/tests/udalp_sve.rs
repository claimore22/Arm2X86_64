/// UADALP (SVE): Unsigned add and accumulate long pairwise (SVE)
/// Stanford source: udalp_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn udalp_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let left = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };
    let right = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 };

    // UADALP (SVE): Unsigned add and accumulate long pairwise
    let result = Expr::ScalableVectorPairwise {
        op: semantic_ir::primitives::SVEVectorPairwiseOp::AddP,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 }),
        vl: BitWidth::W128,
        esize: BitWidth::W64,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UADALP (SVE)".to_string(),
            encoding: Some("Unsigned add and accumulate long pairwise (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "udalp_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn udalp_sve_validates() {
    let instruction = udalp_sve_instruction();
    validate_instruction(&instruction).expect("UADALP (SVE) should validate");
}

#[test]
fn udalp_sve_uses_scalable_pairwise() {
    let instruction = udalp_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorPairwise { op, left, right, destination, vl, esize, predication, .. } = value else {
        panic!("expected ScalableVectorPairwise for UADALP (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorPairwiseOp::AddP);
    assert_eq!(*vl, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}