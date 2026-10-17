/// LDAPR (Load-Acquire Register Pair) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldapr.md
/// Load acquire register pair. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldapr_instruction(mnemonic: &str) -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let data1 = Expr::MemRead {
        address: Box::new(base.clone()),
        size: MemSize::Bits64,
    };
    let address2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base.clone()),
        right: Box::new(Expr::Const {
            value: 8,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let data2 = Expr::MemRead {
        address: Box::new(address2),
        size: MemSize::Bits64,
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: data1,
        },
        Stmt::WriteReg {
            reg: Reg::X(3),
            value: data2,
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-Acquire Register Pair".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldapr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldapr_validates() {
    validate_instruction(&ldapr_instruction("LDAPR"))
        .expect("LDAPR should validate");
}