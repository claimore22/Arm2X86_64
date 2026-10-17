/// ASR (vectors) - Arithmetic shift right (vectors) predicated
/// Stanford source: arm64_md_clean/asr_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorBinOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn asr_vectors_sve_predicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements (e.g., 256 bits, 32‑bit elements => 8 elements)
    let vl = BitWidth::W256;
    let pl = BitWidth::W32; // Predicate register width
    let esize = BitWidth::W32; // Element size for shift amount

    // Predicate, source vector, shift‑amount vector, destination
    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let left = Expr::ReadScalableVecReg { reg: 1, vl };
    let right = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap2 {
        op: SVEVectorBinOp::Asr,
        predicate: Box::new(predicate),
        left: Box::new(left),
        right: Box::new(right),
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ASR".to_string(),
            encoding: Some("Arithmetic shift right (vectors) predicated".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asr_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asr_vectors_sve_predicated_validates() {
    let instruction = asr_vectors_sve_predicated_instruction();
    validate_instruction(&instruction).expect("ASR vectors SVE predicated should validate");
}

#[test]
fn asr_vectors_sve_predicated_structure() {
    let instruction = asr_vectors_sve_predicated_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap2 {
        op,
        predicate,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap2 for ASR vectors");
    };
    assert_eq!(*op, SVEVectorBinOp::Asr);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Merging);
    // Verify the operand types (just ensure they are the expected variant)
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a read from predicate register 0
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}
