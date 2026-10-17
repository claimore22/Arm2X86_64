/// TLBIP (TLB invalidate pair, alias of SYSP) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/tlbip_sysp.md
///
/// SYSP alias (128-bit). Same SysOp/SysLOp model as the earlier SYS aliases.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn tlbip_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let sys_op = Expr::SysOp {
        op0: Box::new(Expr::Const { value: 0b01, ty: BitWidth::new(2).unwrap() }),
        op1: Box::new(Expr::Const { value: 0b011, ty: BitWidth::new(3).unwrap() }),
        crn: Box::new(Expr::Const { value: 0b1000, ty: BitWidth::new(4).unwrap() }),
        crm: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        op2: Box::new(Expr::Const { value: 0b001, ty: BitWidth::new(3).unwrap() }),
        address: Box::new(address),
    };
    let semantics = vec![Stmt::Discard { value: sys_op }];

    Instruction {
        id: InstructionId { mnemonic: "TLBIP".to_string(), encoding: Some("TLB invalidate pair".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_SYSINSTR128".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "tlbip_sysp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn tlbip_validates() {
    validate_instruction(&tlbip_instruction()).expect("should validate");
}
