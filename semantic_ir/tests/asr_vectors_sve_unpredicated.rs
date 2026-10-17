/// ASR (vectors) - Arithmetic shift right (vectors) unpredicated
/// Stanford source: arm64_md_clean/asr_z_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorBinOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn asr_vectors_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Element size for shift amounts

    // No predicate (zeroing mode) – use a constant 0 predicate
    let predicate = Expr::Const { value: 0, ty: BitWidth::W32 };
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
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ASR".to_string(),
            encoding: Some("Arithmetic shift right (vectors) unpredicated".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asr_z_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asr_vectors_sve_unpredicated_validates() {
    let instruction = asr_vectors_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("ASR vectors SVE unpredicated should validate");
}

#[test]
fn asr_vectors_sve_unpredicated_structure() {
    let instruction = asr_vectors_sve_unpredicated_instruction();
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
    assert_eq!(*predication, PredicationMode::Zeroing);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a constant zero
    matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 });
}
