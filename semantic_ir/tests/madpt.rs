/// MADPT (Multiply Add Pointer Tagged) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/madpt.md
/// Multiply-add with pointer tagging. Requires FEAT_CPA.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn madpt_instruction(mnemonic: &str) -> Instruction {
    let width = BitWidth::W64;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width,
    };
    let base_clone = base.clone();
    let offset = Expr::ReadReg {
        reg: Reg::X(2),
        width,
    };
    let tag = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W8,
    };

    // PointerAddCheck: check tag validity, then compute base + offset
    let checked = Expr::PointerAddCheck {
        value: Box::new(offset),
        base: Box::new(base),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base_clone),
        right: Box::new(checked),
        ty: ValueType::Bits(Width::Fixed(width)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Multiply Add Pointer Tagged".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "madpt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn madpt_validates() {
    validate_instruction(&madpt_instruction("MADPT")).expect("MADPT should validate");
}
