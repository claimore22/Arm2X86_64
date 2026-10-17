/// IRG (Insert random tag) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/irg.md
///
/// exclude = Xm[15:0] OR GCR_EL1.Exclude (handled in lowering);
/// rtag = AArch64_ChooseTagOrZero(exclude);
/// Xd = AArch64_AddressWithAllocationTag(Xn, rtag) i.e. address with bits
/// 59:56 replaced by the chosen tag. Modeled with opaque
/// `Expr::ChooseTagOrZero` plus Extract/And/Or/Shift composition.
/// Requires FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, ShiftKind, Stmt, ValueType,
    Width, validate_instruction,
};

fn irg_instruction() -> Instruction {
    let operand = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let exclude_reg = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    // 4-bit random tag not in the exclude set
    let rtag = Expr::ChooseTagOrZero {
        exclude: Box::new(exclude_reg),
    };

    let rtag64 = Expr::ZeroExtend {
        value: Box::new(rtag),
        to: BitWidth::W64,
    };

    // tag << 56
    let tag_in_place = Expr::Shift {
        kind: ShiftKind::Lsl,
        value: Box::new(rtag64),
        amount: Box::new(Expr::Const {
            value: 56,
            ty: BitWidth::W64,
        }),
        ty: BitWidth::W64,
    };

    // address with bits 59:56 cleared
    let cleared = Expr::Binary {
        op: semantic_ir::BinOp::And,
        left: Box::new(operand),
        right: Box::new(Expr::Const {
            value: 0xF0FF_FFFF_FFFF_FFFF,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Xd = cleared OR (tag << 56)
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Or,
        left: Box::new(cleared),
        right: Box::new(tag_in_place),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "IRG".to_string(),
            encoding: Some("Insert random tag".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "irg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn irg_validates() {
    let instruction = irg_instruction();
    validate_instruction(&instruction).expect("IRG should validate");
}

#[test]
fn irg_requires_feat_mte() {
    assert!(irg_instruction()
        .required_features
        .contains(&"FEAT_MTE".to_string()));
}

#[test]
fn irg_uses_random_tag_selection() {
    let instruction = irg_instruction();

    fn find_tag_choose(expr: &Expr) -> bool {
        match expr {
            Expr::ChooseTagOrZero { .. } => true,
            Expr::Binary { left, right, .. } => find_tag_choose(left) || find_tag_choose(right),
            Expr::ZeroExtend { value, .. } => find_tag_choose(value),
            Expr::Shift { value, amount, .. } => find_tag_choose(value) || find_tag_choose(amount),
            _ => false,
        }
    }

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(find_tag_choose(value), "expected ChooseTagOrZero in result");
}

#[test]
fn irg_writes_destination() {
    let instruction = irg_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(1), .. }
    ));
}
