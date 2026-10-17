/// GCSPOPM (Guarded Control Stack pop) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcspopm_sysl.md
///
/// GCSPOPM is an alias of SYSL. Modeled with the existing opaque
/// `Expr::SysLOp` result-valued node. Requires FEAT_GCS.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn gcspopm_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysLOp {
        op0: Box::new(Expr::Const { value: 0b01, ty: BitWidth::new(2).unwrap() }),
        op1: Box::new(Expr::Const { value: 0b000, ty: BitWidth::new(3).unwrap() }),
        crn: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        crm: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        op2: Box::new(Expr::Const { value: 0b001, ty: BitWidth::new(3).unwrap() }),
        address: Box::new(address),
    };

    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: sys_op }];

    Instruction {
        id: InstructionId {
            mnemonic: "GCSPOPM".to_string(),
            encoding: Some("Guarded Control Stack pop (alias of SYSL)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcspopm_sysl.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcspopm_validates() {
    let instruction = gcspopm_instruction();
    validate_instruction(&instruction).expect("GCSPOPM should validate");
}

#[test]
fn gcspopm_requires_feat_gcs() {
    assert!(gcspopm_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcspopm_uses_sys_encoding() {
    let instruction = gcspopm_instruction();
        let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(value, Expr::SysLOp { .. }));
}
