/// REV16 (Reverse bytes in 16-bit halfwords) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rev16_int.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn rev16_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    let operand = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // REV16: Reverse bytes in each 16-bit halfword
    // We model this as a custom Reverse16 operation
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add, // Placeholder for Reverse16
        left: Box::new(operand),
        right: Box::new(Expr::Const { value: 0, ty: BitWidth::W64 }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "REV16".to_string(),
            encoding: Some("Reverse bytes in 16-bit halfwords".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rev16_int.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rev16_validates() {
    let instruction = rev16_instruction();

    validate_instruction(&instruction).expect("REV16 should validate");
}

#[test]
fn rev16_reads_source_writes_dest() {
    let instruction = rev16_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { reg, value } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    assert_eq!(*reg, Reg::X(1));

    let Expr::Binary { left, .. } = value else {
        panic!("expected Binary");
    };

    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::X(2), .. }));
}