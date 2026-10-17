/// AT semantics are representable in the IR.
/// Stanford source: arm64_md_clean/at_sys.md
///
/// AT is an alias of SYS (address translate).
/// Stanford: AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t)
/// with AT-specific fixed fields. Address operand is Xt.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn at_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(5),
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const {
            value: 0b01,
            ty: BitWidth::new(2).unwrap(),
        }),
        op1: Box::new(Expr::Const {
            value: 0, // AT-specific op1
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 7, // AT-specific CRn
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 8, // AT-specific CRm
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 0, // AT-specific op2
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "AT".to_string(),
            encoding: Some("Address translate (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
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
            source_path: "at_sys.md".to_string(),
            source_sha256:
                "badb67dc83fedfb07f2194325a429891b4619a2975ef0abd82b8badf3b12aa2d".to_string(),
            clean_sha256:
                "7d146d213ce63644cd7a32c82215b50f44fbb290959d144f938c2b67d7149a73".to_string(),
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
fn at_validates() {
    let instruction = at_instruction();
    validate_instruction(&instruction).expect("AT should validate");
}

#[test]
fn at_is_alias_of_sys() {
    let instruction = at_instruction();
    let decode = instruction.decode.expect("AT needs decode bindings");
    assert_eq!(decode.statements.len(), 5);
}

#[test]
fn at_discards_sys_op() {
    let instruction = at_instruction();
    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard for SYS alias");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}

#[test]
fn at_sys_op_has_correct_fields() {
    let instruction = at_instruction();
    let (op0, op1, crn, crm, op2, address) = find_sys_op(&instruction.semantics[0])
        .expect("semantics must contain SysOp");

    assert!(matches!(op0.as_ref(), Expr::Const { value: 0b01, ty, .. } if *ty == BitWidth::new(2).unwrap()));
    assert!(matches!(op1.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));
    assert!(matches!(crn.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(crm.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(op2.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(_), width: BitWidth::W64, .. }));
}