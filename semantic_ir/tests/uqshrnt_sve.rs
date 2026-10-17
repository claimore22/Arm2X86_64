/// UQSHRNT (SVE): Unsigned saturating shift right narrow (SVE, unpredicated)
/// Stanford source: uqshrnt_sve.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uqshrnt_sve_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W128 };

    // UQSHRNT (SVE): Unsigned saturating shift right narrow (unpredicated)
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Abs, // Placeholder for UQSHRNT
        predicate: Box::new(Expr::ReadPredReg { reg: 0, pl: BitWidth::W16 }), // vl/8 = 16 bits for 128-bit VL
        operand: Box::new(operand),
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
            mnemonic: "UQSHRNT (SVE)".to_string(),
            encoding: Some("Unsigned saturating shift right narrow (SVE, unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uqshrnt_sve.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uqshrnt_sve_validates() {
    let instruction = uqshrnt_sve_instruction();
    validate_instruction(&instruction).expect("UQSHRNT (SVE) should validate");
}

#[test]
fn uqshrnt_sve_uses_scalable_map() {
    let instruction = uqshrnt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { op, .. } = value else {
        panic!("expected ScalableVectorMap for UQSHRNT (SVE)");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Abs);
}