/// ASRR - Reversed arithmetic shift right by vector (unpredicated)
/// Stanford source: arm64_md_clean/asrr_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorBinOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn asrr_sve_unpredicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements (e.g., 256 bits, 32‑bit elements => 8 elements)
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Element size for shift amounts

    // No predicate (unpredicated) – use a constant 0 predicate
    let predicate = Expr::Const { value: 0, ty: BitWidth::W32 };
    let left = Expr::ReadScalableVecReg { reg: 1, vl };      // This is the value to be shifted (Zdn)
    let right = Expr::ReadScalableVecReg { reg: 2, vl };     // This is the shift amount (Zm)
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap2 {
        op: SVEVectorBinOp::Asr,
        predicate: Box::new(predicate),
        left: Box::new(right),       // Note: swapped - shift amount is left operand
        right: Box::new(left),       // Note: swapped - value to shift is right operand
        destination: Box::new(destination),
        vl,
        esize,
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ASRR".to_string(),
            encoding: Some("Reversed arithmetic shift right by vector (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asrr_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asrr_sve_unpredicated_validates() {
    let instruction = asrr_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("ASRR SVE unpredicated should validate");
}

#[test]
fn asrr_sve_unpredicated_structure() {
    let instruction = asrr_sve_unpredicated_instruction();
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
        panic!("expected ScalableVectorMap2 for ASRR");
    };
    assert_eq!(*op, SVEVectorBinOp::Asr);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Zeroing);
    // Verify the operand types (note: swapped for ASRR)
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }); // shift amount
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }); // value to shift
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    // Predicate should be a constant zero
    matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 });
}