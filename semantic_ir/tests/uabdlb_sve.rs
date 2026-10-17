/// UABDLB (SVE): Unsigned absolute difference long (SVE, predicated)
/// Stanford source: uabdlb_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uabdlb_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };
    let right = Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W128 };

    // UABDLB (SVE): Unsigned absolute difference long
    let sub = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 }),
        vl: BitWidth::W128,
        esize: BitWidth::W32,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };
    let abs_sub = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Abs,
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        operand: Box::new(sub),
        destination: Box::new(Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W128 }),
        vl: BitWidth::W128,
        esize: BitWidth::W32,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: abs_sub,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UABDLB (SVE)".to_string(),
            encoding: Some("Unsigned absolute difference long (SVE)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uabdlb_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uabdlb_sve_validates() {
    let instruction = uabdlb_sve_instruction();
    validate_instruction(&instruction).expect("UABDLB (SVE) should validate");
}

#[test]
fn uabdlb_sve_uses_scalable_ops() {
    let instruction = uabdlb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { op, .. } = value else {
        panic!("expected ScalableVectorMap for Abs");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Abs);
}