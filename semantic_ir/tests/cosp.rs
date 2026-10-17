/// COSP semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cosp_sys.md
///
/// COSP is an alias of SYS (Clear other speculative prediction restriction by context).
/// Stanford: AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t)
/// with fixed fields for the COSP encoding. No address operand (uses zero placeholder).
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn cosp_instruction() -> Instruction {
    // COSP RCTX encoding: op0='01', op1='001', CRn=7, CRm=0, op2='001', t=Rt
    // The address value comes from the Xt register (t = UInt(Rt)).
    // COSP uses a zero placeholder since it doesn't use the address.
    let address = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const {
            value: 0b01,
            ty: BitWidth::new(2).unwrap(),
        }),
        op1: Box::new(Expr::Const {
            value: 0b001,
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 7,
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 0b001,
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "COSP".to_string(),
            encoding: Some("Clear other speculative prediction restriction by context (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES2".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "t".to_string(),
                    value: DecodeExpr::UInt("Rt".to_string()),
                },
                DecodeStmt::Let {
                    name: "op1".to_string(),
                    value: DecodeExpr::UInt("op1".to_string()),
                },
                DecodeStmt::Let {
                    name: "crn".to_string(),
                    value: DecodeExpr::UInt("CRn".to_string()),
                },
                DecodeStmt::Let {
                    name: "crm".to_string(),
                    value: DecodeExpr::UInt("CRm".to_string()),
                },
                DecodeStmt::Let {
                    name: "op2".to_string(),
                    value: DecodeExpr::UInt("op2".to_string()),
                },
            ],
        }),
        semantics: vec![Stmt::Discard { value: sys_op }],
        provenance: Provenance {
            source_path: "cosp_sys.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn find_sys_op(stmt: &Stmt) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match stmt {
        Stmt::Discard { value } => find_sys_op_expr(value),
        _ => None,
    }
}

fn find_sys_op_expr(expr: &Expr) -> Option<(&Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>, &Box<Expr>)> {
    match expr {
        Expr::SysOp { op0, op1, crn, crm, op2, address } => Some((op0, op1, crn, crm, op2, address)),
        _ => None,
    }
}

#[test]
fn cosp_validates() {
    let instruction = cosp_instruction();
    validate_instruction(&instruction).expect("COSP should validate");
}

#[test]
fn cosp_requires_specreS2_feature() {
    let instruction = cosp_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_SPECRES2".to_string()]);
}

#[test]
fn cosp_is_alias_of_sys() {
    let instruction = cosp_instruction();
    let decode = instruction.decode.expect("COSP needs decode bindings");
    assert_eq!(decode.statements.len(), 5); // t, op1, crn, crm, op2
}

#[test]
fn cosp_discards_sys_op() {
    let instruction = cosp_instruction();
    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard for SYS alias");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}

#[test]
fn cosp_sys_op_has_correct_fields() {
    let instruction = cosp_instruction();
    let (op0, op1, crn, crm, op2, address) = find_sys_op(&instruction.semantics[0])
        .expect("semantics must contain SysOp");

    // op0 = '01' (2 bits)
    assert!(matches!(op0.as_ref(), Expr::Const { value: 0b01, ty, .. } if *ty == BitWidth::new(2).unwrap()));

    // op1 = '001' (3 bits)
    assert!(matches!(op1.as_ref(), Expr::Const { value: 0b001, ty, .. } if *ty == BitWidth::new(3).unwrap()));

    // crn = 7 (4 bits)
    assert!(matches!(crn.as_ref(), Expr::Const { value: 7, ty, .. } if *ty == BitWidth::new(4).unwrap()));

    // crm = 0 (4 bits)
    assert!(matches!(crm.as_ref(), Expr::Const { value: 0, ty, .. } if *ty == BitWidth::new(4).unwrap()));

    // op2 = '001' (3 bits)
    assert!(matches!(op2.as_ref(), Expr::Const { value: 0b001, ty, .. } if *ty == BitWidth::new(3).unwrap()));

    // address = zero placeholder
    assert!(matches!(address.as_ref(), Expr::Const { value: 0, ty, .. } if *ty == BitWidth::W64));
}