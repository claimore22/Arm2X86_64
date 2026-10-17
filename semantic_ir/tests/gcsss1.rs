/// GCSSS1 (Guarded Control Stack switch stack 1) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gcsss1_sys.md
///
/// GCSSS1 is an alias of SYS. Modeled with the existing opaque
/// `Expr::SysOp` statement-level node. Requires FEAT_GCS.
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn gcsss1_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const { value: 0b01, ty: BitWidth::new(2).unwrap() }),
        op1: Box::new(Expr::Const { value: 0b000, ty: BitWidth::new(3).unwrap() }),
        crn: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        crm: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        op2: Box::new(Expr::Const { value: 0b101, ty: BitWidth::new(3).unwrap() }),
        address: Box::new(address),
    };

    let semantics = vec![Stmt::Discard { value: sys_op }];

    Instruction {
        id: InstructionId {
            mnemonic: "GCSSS1".to_string(),
            encoding: Some("Guarded Control Stack switch stack 1 (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_GCS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gcsss1_sys.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gcsss1_validates() {
    let instruction = gcsss1_instruction();
    validate_instruction(&instruction).expect("GCSSS1 should validate");
}

#[test]
fn gcsss1_requires_feat_gcs() {
    assert!(gcsss1_instruction()
        .required_features
        .contains(&"FEAT_GCS".to_string()));
}

#[test]
fn gcsss1_uses_sys_encoding() {
    let instruction = gcsss1_instruction();
        let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}
