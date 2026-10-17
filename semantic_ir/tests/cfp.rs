/// CFP semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cfp_sys.md
///
/// CFP is an alias of SYS (Control flow prediction restriction by context).
/// Stanford: AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t)
/// with fixed fields for the CFP encoding: SYS #3, C7, C3, #4, Xt.
/// The address operand is Xt.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn cpf_instruction() -> Instruction {
    // CFP encoding: fixed op0='01', op1=3, crn=7, crm=3, op2=4, t=Rt
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
            value: 3, // CFP-specific op1 = #3
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 7, // CFP-specific CRn = C7
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 3, // CFP-specific CRm = C3
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 4, // CFP-specific op2 = #4
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    Instruction {
        id: InstructionId {
            mnemonic: "CFP".to_string(),
            encoding: Some("Control flow prediction restriction by context (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SPECRES".to_string()],
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
            source_path: "cfp_sys.md".to_string(),
            source_sha256: "96afb09d40f6f58e0af26b2f07f855cb74aadf427adc940f8e094a52401d532a".to_string(),
            clean_sha256: "8b0370876c23c3bff79d7add6474bbee8ad9c580a9809a47db6141b705edf7c0".to_string(),
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
fn cpf_validates() {
    let instruction = cpf_instruction();
    validate_instruction(&instruction).expect("CFP should validate");
}

#[test]
fn cpf_requires_specret_feature() {
    let instruction = cpf_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_SPECRES".to_string()]);
}

#[test]
fn cpf_is_alias_of_sys() {
    let instruction = cpf_instruction();
    // CFP is an alias - decode has fixed fields
    let decode = instruction.decode.expect("CFP needs decode bindings");
    assert_eq!(decode.statements.len(), 5); // t, op1, crn, crm, op2
}

#[test]
fn cpf_discards_sys_op() {
    let instruction = cpf_instruction();
    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard for SYS alias");
    };
    assert!(matches!(value, Expr::SysOp { .. }));
}

#[test]
fn cpf_sys_op_has_correct_fields() {
    let instruction = cpf_instruction();
    let (op0, op1, crn, crm, op2, address) = find_sys_op(&instruction.semantics[0])
        .expect("semantics must contain SysOp");

    // op0 = '01' (2 bits)
    assert!(matches!(op0.as_ref(), Expr::Const { value: 0b01, ty, .. } if *ty == BitWidth::new(2).unwrap()));

    // op1 = 3 (3 bits)
    assert!(matches!(op1.as_ref(), Expr::Const { value: 3, ty, .. } if *ty == BitWidth::new(3).unwrap()));

    // crn = 7 (4 bits)
    assert!(matches!(crn.as_ref(), Expr::Const { value: 7, ty, .. } if *ty == BitWidth::new(4).unwrap()));

    // crm = 3 (4 bits)
    assert!(matches!(crm.as_ref(), Expr::Const { value: 3, ty, .. } if *ty == BitWidth::new(4).unwrap()));

    // op2 = 4 (3 bits)
    assert!(matches!(op2.as_ref(), Expr::Const { value: 4, ty, .. } if *ty == BitWidth::new(3).unwrap()));

    // address = Xt register
    assert!(matches!(address.as_ref(), Expr::ReadReg { reg: Reg::X(_), width: BitWidth::W64, .. }));
}