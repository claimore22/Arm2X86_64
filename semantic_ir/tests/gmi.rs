/// GMI (Tag mask insert) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/gmi.md
///
/// mask[UInt(tag)] = '1' where tag = AArch64_AllocationTagFromAddress(address)
/// (bits 59:56). Modeled as: Xd = Xm OR (1 << UInt(tag)).
/// Requires FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn gmi_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    // tag = address[59:56] (AArch64_AllocationTagFromAddress)
    let tag = Expr::Extract {
        value: Box::new(address),
        lsb: 56,
        width: BitWidth::W4,
    };

    let tag64 = Expr::ZeroExtend {
        value: Box::new(tag),
        to: BitWidth::W64,
    };

    let one = Expr::Const {
        value: 1,
        ty: BitWidth::W64,
    };

    // shifted = 1 << UInt(tag)
    let shifted = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(one),
        amount: Box::new(tag64),
        ty: BitWidth::W64,
    };

    let mask = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // Xd = mask OR (1 << tag)
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(mask),
        right: Box::new(shifted),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "GMI".to_string(),
            encoding: Some("Tag mask insert".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "gmi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn gmi_validates() {
    let instruction = gmi_instruction();
    validate_instruction(&instruction).expect("GMI should validate");
}

#[test]
fn gmi_requires_feat_mte() {
    assert!(gmi_instruction()
        .required_features
        .contains(&"FEAT_MTE".to_string()));
}

#[test]
fn gmi_sets_tag_bit_in_mask() {
    let instruction = gmi_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op, right, .. } = value else {
        panic!("expected Binary Or");
    };

    assert_eq!(*op, semantic_ir::BinOp::Or);
    assert!(matches!(
        right.as_ref(),
        Expr::Shift {
            kind: ShiftKind::Lsl,
..
        }
    ));
}
