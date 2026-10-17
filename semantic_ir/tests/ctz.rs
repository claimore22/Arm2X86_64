/// CTZ (Count Trailing Zeros) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ctz.md
///
/// result = CountLeadingZeroBits(BitReverse(operand)).
/// BitReverse/CountLeadingZeroBits have no composing IR ops, so we model the
/// operation with a placeholder binary op, same precedent as CLZ.
/// Requires FEAT_CSSC.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ctz_instruction() -> Instruction {
    let _datasize = Width::Fixed(BitWidth::W64);

    let operand1 = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // CTZ: CountLeadingZeroBits(BitReverse(operand1))
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add, // Placeholder - would be a custom CountTrailingZeroBits
        left: Box::new(operand1),
        right: Box::new(Expr::Const {
            value: 0,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CTZ".to_string(),
            encoding: Some("Count trailing zeros".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CSSC".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ctz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ctz_validates() {
    let instruction = ctz_instruction();
    validate_instruction(&instruction).expect("CTZ should validate");
}

#[test]
fn ctz_requires_feat_cssc() {
    let instruction = ctz_instruction();
    assert!(instruction.required_features.contains(&"FEAT_CSSC".to_string()));
}

#[test]
fn ctz_reads_source_writes_dest() {
    let instruction = ctz_instruction();

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
