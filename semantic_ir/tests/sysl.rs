/// SYSL (System instruction with result) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/sysl.md
///
/// AArch64_SysInstrWithResult: returns 64-bit result in Xt. Same SysOp/SysLOp model as the earlier SYS aliases.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sysl_instruction() -> Instruction {
    let address = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let sys_op = Expr::SysLOp {
        op0: Box::new(Expr::Const { value: 0b01, ty: BitWidth::new(2).unwrap() }),
        op1: Box::new(Expr::Const { value: 0b011, ty: BitWidth::new(3).unwrap() }),
        crn: Box::new(Expr::Const { value: 0b1000, ty: BitWidth::new(4).unwrap() }),
        crm: Box::new(Expr::Const { value: 0b0111, ty: BitWidth::new(4).unwrap() }),
        op2: Box::new(Expr::Const { value: 0b001, ty: BitWidth::new(3).unwrap() }),
        address: Box::new(address),
    };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: sys_op }];

    Instruction {
        id: InstructionId { mnemonic: "SYSL".to_string(), encoding: Some("System instruction with result".to_string()) },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "sysl.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn sysl_validates() {
    validate_instruction(&sysl_instruction()).expect("should validate");
}
