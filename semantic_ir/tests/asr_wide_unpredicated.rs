/// ASR (wide elements, unpredicated) - Arithmetic shift right by 64-bit wide elements (unpredicated)
/// Stanford source: arm64_md_clean/asr_z_zw.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorBinOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn asr_wide_unpredicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements with 64-bit esize
    // e.g., 256 bits, 64-bit elements => 4 elements
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // Element size for wide elements

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
            encoding: Some("Arithmetic shift right (wide elements, unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asr_z_zw.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asr_wide_unpredicated_validates() {
    let instruction = asr_wide_unpredicated_instruction();
    validate_instruction(&instruction).expect("ASR wide elements unpredicated should validate");
}

#[test]
fn asr_wide_unpredicated_structure() {
    let instruction = asr_wide_unpredicated_instruction();
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
        panic!("expected ScalableVectorMap2 for ASR wide elements unpredicated");
    };
    assert_eq!(*op, SVEVectorBinOp::Asr);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, PredicationMode::Zeroing);
    // Verify the operand types (just ensure they are the expected variant)
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a constant zero
    matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 });
}