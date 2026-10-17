/// SUQADD (SVE): Signed saturating accumulate of unsigned value (SVE)
/// Stanford source: suqadd_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn suqadd_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };
    let right = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 };

    // SUQADD (SVE): Signed saturating accumulate of unsigned value
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::SUQAdd,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }),
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
            mnemonic: "SUQADD (SVE)".to_string(),
            encoding: Some("Signed saturating accumulate of unsigned value (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "suqadd_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn suqadd_sve_validates() {
    let instruction = suqadd_sve_instruction();
    validate_instruction(&instruction).expect("SUQADD (SVE) should validate");
}

#[test]
fn suqadd_sve_uses_scalable_vector_map2() {
    let instruction = suqadd_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, predicate, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for SUQADD (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::SUQAdd);
    assert_eq!(*vl, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}