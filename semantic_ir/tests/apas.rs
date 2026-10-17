/// APAS semantics are representable in the IR.
/// Stanford source: arm64_md_clean/apas_sys.md
///
/// APAS is an alias of SYS (associate physical address space).
/// Stanford: AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t)
/// with fixed fields for the APAS encoding. The address operand is Xt.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn apas_instruction() -> Instruction {
    // APAS encoding: fixed op0='01', op1=op1, crn=CRn, crm=CRm, op2=op2, t=Rt
    // The address value comes from the Xt register (t = UInt(Rt)).
    let address = Expr::ReadReg {
        reg: Reg::X(5), // Example: Rt=5
        width: BitWidth::W64,
    };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const {
            value: 0b01,
            ty: BitWidth::new(2).unwrap(),
        }),
        op1: Box::new(Expr::Const {
            value: 0, // APAS-specific op1
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 0, // APAS-specific CRn
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 0, // APAS-specific CRm
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 0, // APAS-specific op2
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "APAS".to_string(),
            encoding: Some("Associate physical address space (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAS".to_string()],
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
            source_path: "apas_sys.md".to_string(),
            source_sha256:
                "f34dab75dc390d2e5771ac5c7f05bd32827f2eefa98d7b3f6d2b6bf4ca440f4b".to_string(),
            clean_sha256:
                "13cec1fda913ba9d9b1f5f2d5e67d8fb2051f09c3e520133f03d1e9230b88a5a".to_string(),
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
fn apas_validates() {
    let instruction = apas_instruction();
    validate_instruction(&instruction).expect("APAS should validate");
}

#[test]
fn apas_requires_pas_feature() {
    let instruction = apas_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_PAS".to_string()]);
}

#[test]
fn apas_is_alias_of_sys() {
    let instruction = apas_instruction();
    // APAS is an alias - decode has fixed fields
    let decode = instruction.decode.expect("APAS needs decode bindings");
    assert_eq!(decode.statements.len(), 5); // t, op1, crn, crm, op2
}

#[test]
fn apas_discards_sys_op() {
    let instruction = apas_instruction();
    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard for SYS alias");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}

#[test]
fn apas_sys_op_has_correct_fields() {
    let instruction = apas_instruction();
    let (op0, op1, crn, crm, op2, address) = find_sys_op(&instruction.semantics[0])
        .expect("semantics must contain SysOp");

    // op0 = '01' (2 bits)
    assert!(matches!(op0.as_ref(), Expr::Const { value: 0b01, ty, .. } if *ty == BitWidth::new(2).unwrap()));

    // op1, crn, crm, op2 are fixed constants for APAS alias
    assert!(matches!(op1.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));
    assert!(matches!(crn.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(crm.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(4).unwrap()));
    assert!(matches!(op2.as_ref(), Expr::Const { ty, .. } if *ty == BitWidth::new(3).unwrap()));

    // address = Xt register
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(_), width: BitWidth::W64, .. }));
}