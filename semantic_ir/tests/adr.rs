/// ADR (Form PC-relative address) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/adr.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn adr_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // imm = SignExtend(immhi::immlo)
    let imm = Expr::Const {
        value: 0x1000,
        ty: BitWidth::W64,
    };

// ADR: Xd = PC + imm
    let pc = Expr::ReadSpecial {
        reg: SpecialReg::Pc64,
        width: BitWidth::W64,
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(pc),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADR".to_string(),
            encoding: Some("Form PC-relative address".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adr_validates() {
    let instruction = adr_instruction();

    validate_instruction(&instruction).expect("ADR should validate");
}

#[test]
fn adr_computes_pc_relative_address() {
    let instruction = adr_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, left, right, .. } = value else {
        panic!("expected Binary Add");
    };

    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(left.as_ref(), Expr::ReadSpecial { reg: semantic_ir::SpecialReg::Pc64, .. }));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}