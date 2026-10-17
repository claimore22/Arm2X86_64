/// SCVTF (predicated) - Signed integer convert to floating-point
/// Stanford source: arm64_md_clean/scvtf_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn scvtf_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Source/destination element size

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // SCVTF: signed integer convert to floating-point (merging)
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Scvtf,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SCVTF (merging)".to_string(),
            encoding: Some("Signed integer convert to floating-point (merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "scvtf_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn scvtf_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // SCVTF: signed integer convert to floating-point (zeroing)
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Scvtf,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SCVTF (zeroing)".to_string(),
            encoding: Some("Signed integer convert to floating-point (zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "scvtf_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn scvtf_sve_merging_validates() {
    let instruction = scvtf_sve_merging_instruction();
    validate_instruction(&instruction).expect("SCVTF SVE merging should validate");
}

#[test]
fn scvtf_sve_zeroing_validates() {
    let instruction = scvtf_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("SCVTF SVE zeroing should validate");
}

#[test]
fn scvtf_sve_merging_structure() {
    let instruction = scvtf_sve_merging_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap { op, predicate, operand, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap for SCVTF merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Scvtf);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}