/// BRB semantics are representable in the IR.
/// Stanford source: arm64_md_clean/brb_sys.md
///
/// BRB is an alias of SYS (branch record buffer).
/// Stanford: AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t)
/// with BRB-specific fixed fields. No address operand (uses implied state).
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn brb_instruction() -> Instruction {
    // BRB has no address operand - it's a system operation on branch record buffer state.
    // We use a zero address as placeholder since SysOp requires an address field.
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
            value: 0, // BRB-specific op1
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 9, // BRB-specific CRn
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 0, // BRB-specific CRm
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 0, // BRB-specific op2
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "BRB".to_string(),
            encoding: Some("Branch record buffer (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BRBE".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "op1".to_string(),
                    value: DecodeExpr::UInt("op1".to_string()),
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
            source_path: "brb_sys.md".to_string(),
            source_sha256:
                "88ede0f8d2f46e5280040e9d39a4dd9d2e25e9ed323ecbcd9e6841f6aa5edabf".to_string(),
            clean_sha256:
                "c12dd8c717eb4ebb53804675e7c18e4c5bf9cfeb3900d143bd211d4cf1280184".to_string(),
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
fn brb_validates() {
    let instruction = brb_instruction();
    validate_instruction(&instruction).expect("BRB should validate");
}

#[test]
fn brb_requires_brbe_feature() {
    let instruction = brb_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_BRBE".to_string()]);
}

#[test]
fn brb_is_alias_of_sys() {
    let instruction = brb_instruction();
    let decode = instruction.decode.expect("BRB needs decode bindings");
    assert_eq!(decode.statements.len(), 3); // op1, crm, op2
}

#[test]
fn brb_discards_sys_op() {
    let instruction = brb_instruction();
    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard for SYS alias");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}

#[test]
fn brb_sys_op_has_correct_fields() {
    let instruction = brb_instruction();
    let (op0, op1, crn, crm, op2, address) = find_sys_op(&instruction.semantics[0])
        .expect("semantics must contain SysOp");

    assert!(matches!(op0.as_ref(), Expr::Const { value: 0b01, ty, .. } if *ty == BitWidth::new(2).unwrap()));
    assert!(matches!(op1.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));
    assert!(matches!(crn.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(crm.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(op2.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));
    assert!(matches!(address.as_ref(), Expr::Const { value: 0, ty: BitWidth::W64, .. }));
}