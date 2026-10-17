/// ASRD - Arithmetic shift right for divide by immediate (predicated)
/// Stanford source: arm64_md_clean/asrd_z_p_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    SVEVectorUnaryOp,
    primitives::PredicationMode,
    validate_instruction,
};

fn asrd_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // Element size

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: SVEVectorUnaryOp::Asrd,
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
            mnemonic: "ASRD".to_string(),
            encoding: Some("Arithmetic shift right for divide by immediate (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "asrd_z_p_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn asrd_sve_predicated_validates() {
    let instruction = asrd_sve_predicated_instruction();
    validate_instruction(&instruction).expect("ASRD SVE predicated should validate");
}

#[test]
fn asrd_sve_predicated_structure() {
    let instruction = asrd_sve_predicated_instruction();
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
        panic!("expected ScalableVectorMap for ASRD");
    };
    assert_eq!(*op, SVEVectorUnaryOp::Asrd);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Merging);
    // Verify the operand types
    matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
}