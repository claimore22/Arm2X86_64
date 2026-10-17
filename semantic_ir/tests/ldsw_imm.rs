/// LD1SW (Load Single Word, scalar plus immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ld1sw.md
/// Load single word with scalar plus immediate addressing. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ld1sw_instruction(mnemonic: &str) -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let offset = Expr::Const {
        value: 4,
        ty: BitWidth::W64,
    };

    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(offset),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(2),
        value: Expr::ZeroExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Single Word".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sw.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1sw_validates() {
    validate_instruction(&ld1sw_instruction("LD1SW")).expect("LD1SW should validate");
}

#[test]
fn ld1sw_scalar_validates() {
    validate_instruction(&ld1sw_instruction("LD1SW"))
        .expect("LD1SW scalar plus immediate should validate");
}
