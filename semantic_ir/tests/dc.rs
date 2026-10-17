/// DC (Data cache operation) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/dc_sys.md
///
/// DC is an alias of SYS (op0 == 0b01, cache maintenance). The exact cache
/// operation is selected by the op1/CRn/CRm/op2 fields. Modeled with the
/// existing opaque `Expr::SysOp` statement-level node (same as AT/APAS).
/// Alias decision recorded in AI_DECISIONS.md.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn dc_instruction() -> Instruction {
    // Example: DC IVAC-style encoding fields (SYS op0=01 cache maintenance)
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
            value: 0, // cache op specific
            ty: BitWidth::new(3).unwrap(),
        }),
        crn: Box::new(Expr::Const {
            value: 7, // cache maintenance CRn
            ty: BitWidth::new(4).unwrap(),
        }),
        crm: Box::new(Expr::Const {
            value: 6, // cache op specific CRm
            ty: BitWidth::new(4).unwrap(),
        }),
        op2: Box::new(Expr::Const {
            value: 1, // cache op specific op2
            ty: BitWidth::new(3).unwrap(),
        }),
        address: Box::new(address),
    };

    // SysOp is statement-level: use Discard (same precedent as AT/APAS)
    let semantics = vec![Stmt::Discard { value: sys_op }];

    Instruction {
        id: InstructionId {
            mnemonic: "DC".to_string(),
            encoding: Some("Data cache operation (alias of SYS)".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dc_sys.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dc_validates() {
    let instruction = dc_instruction();
    validate_instruction(&instruction).expect("DC should validate");
}

#[test]
fn dc_is_sys_alias_with_op0_01() {
    let instruction = dc_instruction();

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

#[test]
fn dc_targets_address_register() {
    let instruction = dc_instruction();

    let Stmt::Discard { value } = &instruction.semantics[0] else {
        panic!("expected Discard");
    };

    let Expr::SysOp { address, .. } = value else {
        panic!("expected SysOp");
    };

    assert!(matches!(
        address.as_ref(),
        Expr::ReadReg { reg: Reg::X(1), .. }
    ));
}
