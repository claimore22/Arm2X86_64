/// FRINT32Z - Floating-point round to 32-bit integer, rounding toward zero (unpredicated)
/// Stanford source: arm64_md_clean/frint32z_z_p_z.md (zeroing predication)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::{PredicationMode, SVEVectorUnaryOp},
    validate_instruction,
};

fn frint32z_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // 32-bit elements for FRINT32Z

    // No predicate (zeroing mode) – use a constant 0 predicate
    let predicate = Expr::Const { value: 0, ty: BitWidth::W32 };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::FRint32Z,
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
            mnemonic: "FRINT32Z".to_string(),
            encoding: Some("Floating-point round to 32-bit integer, rounding toward zero (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frint32z_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frint32z_sve_unpredicated_validates() {
    let instruction = frint32z_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FRINT32Z SVE unpredicated should validate");
}

#[test]
fn frint32z_sve_unpredicated_structure() {
    let instruction = frint32z_sve_unpredicated_instruction();
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
        panic!("expected ScalableVectorMap for FRINT32Z");
    };
    assert_eq!(*op, SVEVectorUnaryOp::FRint32Z);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Zeroing);
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::Const { value: 0, ty: BitWidth::W32 });
}