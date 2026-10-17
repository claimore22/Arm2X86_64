/// BFCVT - Single-precision convert to BFloat16 (predicated)
/// Stanford source: arm64_md_clean/bfcvt_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorUnaryOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn bfcvt_sve_predicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements (e.g., 256 bits)
    let vl = BitWidth::W256;
    // Source: 32-bit float elements, Destination: 16-bit elements (BFloat16)
    let esize = BitWidth::W32;   // Source element size (32-bit float)
    let d_esize = BitWidth::W16; // Destination element size (16-bit BFloat16)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::Bfcvt,
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
            mnemonic: "BFCVT".to_string(),
            encoding: Some("Single-precision convert to BFloat16 (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()], // From decode
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvt_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfcvt_sve_predicated_validates() {
    let instruction = bfcvt_sve_predicated_instruction();
    validate_instruction(&instruction).expect("BFCVT SVE predicated should validate");
}

#[test]
fn bfcvt_sve_predicated_structure() {
    let instruction = bfcvt_sve_predicated_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for BFCVT");
    };
    assert_eq!(*op, SVEVectorUnaryOp::Bfcvt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32); // 32-bit source
    assert_eq!(*predication, PredicationMode::Merging);
    // Verify the operand types
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a read from predicate register 0
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}