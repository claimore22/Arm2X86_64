/// FCVTNT (predicated) - Floating-point narrowing convert (top, predicated)
/// Stanford source: arm64_md_clean/fcvtnt_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcvtnt_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Destination element size (32-bit single-precision)
    let src_esize = BitWidth::W64; // Source element size (64-bit double-precision)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n] - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[d] - destination

    // FCVTNT: narrowing convert from double-precision to single-precision (top elements)
    // Result goes to odd-numbered half-width elements (top half of each pair)
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvtnt,
        predicate: Box::new(predicate),
        operand: Box::new(operand), // Source operand
        destination: Box::new(destination),
        vl,
        esize, // Destination element size (32-bit)
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCVTNT (merging)".to_string(),
            encoding: Some("Floating-point narrowing convert top (merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtn_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcvtnt_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;
    let src_esize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Fcvtnt,
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
            mnemonic: "FCVTNT (zeroing)".to_string(),
            encoding: Some("Floating-point narrowing convert top (zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcvtn_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcvtnt_sve_merging_validates() {
    let instruction = fcvtnt_sve_merging_instruction();
    validate_instruction(&instruction).expect("FCVTNT SVE merging should validate");
}

#[test]
fn fcvtnt_sve_zeroing_validates() {
    let instruction = fcvtnt_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FCVTNT SVE zeroing should validate");
}

#[test]
fn fcvtnt_sve_merging_structure() {
    let instruction = fcvtnt_sve_merging_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap { op, predicate, operand, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap for FCVTNT merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Fcvtnt);
    matches!(&**predicate, Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}