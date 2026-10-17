/// LDRSW (Load Register Signed Word) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldrsw.md
/// Load signed word immediate.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldrsw_instruction(mnemonic: &str) -> Instruction {
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
        value: Expr::SignExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Register Signed Word".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldrsw.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldrsw_validates() {
    validate_instruction(&ldrsw_instruction("LDRSW")).expect("LDRSW should validate");
}
