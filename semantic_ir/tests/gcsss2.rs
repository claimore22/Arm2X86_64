/// GCSSS2 (Guarded Control Stack switch stack 2) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcsss2_sysl.md
///
/// GCSSS2 is an alias of SYSL. Modeled with the existing opaque
/// `Expr::SysLOp` result-valued node. Requires FEAT_GCS.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn gcsss2_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysLOp {
        op0: Box::new(Expr::Const { value: 0b01, ty: BitWidth::new(2).unwrap() }),
        op1: Box::new(Expr::Const { value: 0b000, ty: BitWidth::new(3).unwrap() }),
        crn: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        crm: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        op2: Box::new(Expr::Const { value: 0b110, ty: BitWidth::new(3).unwrap() }),
        address: Box::new(address),
    };

    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: sys_op }];

    Instruction {
        id: InstructionId {
            mnemonic: "GCSSS2".to_string(),
            encoding: Some("Guarded Control Stack switch stack 2 (alias of SYSL)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcsss2_sysl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcsss2_validates() {
    let instruction = gcsss2_instruction();
    validate_instruction(&instruction).expect("GCSSS2 should validate");
}

#[test]
fn gcsss2_requires_feat_gcs() {
    assert!(gcsss2_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcsss2_uses_sys_encoding() {
    let instruction = gcsss2_instruction();
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::SysLOp { .. }));
}
