/// DVP (Data value prediction restriction by context) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dvp_sys.md
///
/// DVP is an alias of SYS (DVP RCTX): op0=0b01, op1=0b011, CRn=0b0111,
/// CRm=0b0011, op2=0b101. Modeled with the existing opaque `Expr::SysOp`
/// statement-level node (same as DC/AT/APAS).
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn dvp_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const {
            value: 0b01,
            ty: BitWidth::new(2).unwrap(),
        }),
        op1: Box::new(Expr::Const {
            value: 0b011, // DVP RCTX op1
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 0b0111, // DVP RCTX CRn
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 0b0011, // DVP RCTX CRm
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 0b101, // DVP RCTX op2
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    let semantics = vec![Stmt::Discard { value: sys_op }];

    Instruction {
        id: InstructionId {
            mnemonic: "DVP".to_string(),
            encoding: Some("Data value prediction restriction (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dvp_sys.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dvp_validates() {
    let instruction = dvp_instruction();
    validate_instruction(&instruction).expect("DVP should validate");
}

#[test]
fn dvp_is_sys_alias_with_op0_01() {
    let instruction = dvp_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard");
    };

    let Expr::SysOp { op0, .. } = value else {
        panic!("expected SysOp");
    };

    assert!(matches!(
        op0.as_ref(),
        Expr::Const { value: 1, ty } if *ty == BitWidth::new(2).unwrap()
    ));
}
