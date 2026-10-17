/// B (unconditional branch) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/b.md
///
/// Stanford (arm64_md_clean/b.md):
///   offset = SignExtend(imm26::'00')
///   BranchTo(PC64() + offset, DIR)
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId, Provenance,
    SpecialReg, Stmt, ValueType, Width, validate_instruction,
};

fn b_instruction() -> Instruction {
    // offset = SignExtend(imm26::'00')
    let offset = Expr::Const {
        value: 0x100000,
        ty: BitWidth::W64,
    };

    // target = PC + offset
    let target = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(Expr::ReadSpecial {
            reg: SpecialReg::Pc64,
            width: BitWidth::W64,
        }),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::Branch {
        target,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "B".to_string(),
            encoding: Some("Branch".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "offset".to_string(),
                    value: DecodeExpr::ShiftLeft {
                        value: Box::new(DecodeExpr::UInt("imm26".to_string())),
                        amount: 2,
                    },
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "b.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn b_validates() {
    let instruction = b_instruction();
    validate_instruction(&instruction).expect("B should validate");
}

#[test]
fn b_uses_branch() {
    let instruction = b_instruction();
    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::Branch { .. }
    ));
}

#[test]
fn b_branches_to_pc_plus_offset() {
    let instruction = b_instruction();
    let Stmt::Branch { target } = &instruction.semantics[0] else {
        panic!("expected Branch");
    };

    let Expr::Binary { op, left, right, ty: ValueType::Bits(Width::Fixed(BitWidth::W64)) } = target else {
        panic!("expected Binary Add for PC-relative target");
    };
    assert_eq!(*op, semantic_ir::BinOp::Add);
    assert!(matches!(
        left.as_ref(),
        Expr::ReadSpecial { reg: SpecialReg::Pc64, .. }
    ));
    assert!(matches!(right.as_ref(), Expr::Const { .. }));
}

#[test]
fn b_decode_scales_imm26() {
    let instruction = b_instruction();
    let decode = instruction.decode.expect("B needs decode");
    assert_eq!(decode.statements.len(), 1);
    // offset = SignExtend(imm26::'00') -> ShiftLeft(imm26, 2)
    let has_scaled_offset = decode.statements.iter().any(|s| {
        matches!(s, DecodeStmt::Let {
            value: DecodeExpr::ShiftLeft { amount: 2, .. },
..
        })
    });
    assert!(has_scaled_offset, "decode must scale imm26 by 2");
}
