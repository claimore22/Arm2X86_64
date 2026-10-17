/// CNOT - Logically invert boolean condition (predicated)
/// Stanford source: arm64_md_clean/cnot_z_p_z.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn cnot_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Not,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        destination: Box::new(destination), vl, esize, predication: semantic_ir::primitives::PredicationMode::Merging, };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "CNOT".to_string(),
            encoding: Some("Logically invert boolean condition (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cnot_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cnot_sve_predicated_validates() {
    let instruction = cnot_sve_predicated_instruction();
    validate_instruction(&instruction).expect("CNOT SVE predicated should validate");
}

#[test]
fn cnot_sve_predicated_uses_scalable_vector_map() {
    let instruction = cnot_sve_predicated_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::ScalableVectorMap { op, predicate, operand, destination, vl, esize, predication } = value else { panic!("expected ScalableVectorMap for CNOT"); };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Not);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}