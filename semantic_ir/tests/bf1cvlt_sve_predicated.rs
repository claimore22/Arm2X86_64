/// BF1CVTLT, BF2CVTLT - 8-bit floating-point convert to BFloat16 (top)
/// Stanford source: arm64_md_clean/bf1cvtlt_z_z8.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorUnaryOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn bf1cvlt_sve_predicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements (e.g., 256 bits)
    let vl = BitWidth::W256;
    // Source: 8-bit elements, Destination: 16-bit elements (BFloat16)
    let esize = BitWidth::W8;   // Source element size (8-bit float)
    let d_esize = BitWidth::W16; // Destination element size (16-bit BFloat16)

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::Bfloat16Cvtlt,
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
            mnemonic: "BF1CVTLT".to_string(),
            encoding: Some("8-bit floating-point convert to BFloat16 (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE2".to_string(), 
            "FEAT_SME2".to_string(),
            "FEAT_FP8".to_string() // From decode: FEAT_FP8
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bf1cvtlt_z_z8.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bf1cvlt_sve_predicated_validates() {
    let instruction = bf1cvlt_sve_predicated_instruction();
    validate_instruction(&instruction).expect("BF1CVTLT SVE predicated should validate");
}

#[test]
fn bf1cvlt_sve_predicated_structure() {
    let instruction = bf1cvlt_sve_predicated_instruction();
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
        panic!("expected ScalableVectorMap for BF1CVTLT");
    };
    assert_eq!(*op, SVEVectorUnaryOp::Bfloat16Cvtlt);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W8); // 8-bit source
    assert_eq!(*predication, PredicationMode::Merging);
    // Verify the operand types
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a read from predicate register 0
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}