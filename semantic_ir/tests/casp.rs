/// CASP (Compare And Swap Pair) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/casp.md
/// Compare and swap pair. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    ValueType, Width, validate_instruction,
};
use semantic_ir::primitives::AtomicRMWOp;

fn casp_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let expected = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };
    let new = Expr::ReadReg {
        reg: Reg::X(3),
        width: BitWidth::W64,
    };

    let old1 = Expr::MemAtomic {
        address: Box::new(address.clone()),
        expected: Box::new(expected.clone()),
        new_value: Box::new(new.clone()),
        size: MemSize::Bits64,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };
    let old2 = Expr::MemAtomic {
        address: Box::new(Expr::Binary {
            op: semantic_ir::BinOp::Add,
            left: Box::new(address),
            right: Box::new(Expr::Const {
                value: 8,
                ty: BitWidth::W64,
            }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        expected: Box::new(expected),
        new_value: Box::new(new),
        size: MemSize::Bits64,
        ordering: semantic_ir::MemOrdering::Relaxed,
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(4),
            value: old1,
        },
        Stmt::WriteReg {
            reg: Reg::X(5),
            value: old2,
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Compare And Swap Pair".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "casp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn casp_validates() {
    validate_instruction(&cfvt_instruction("CASP"))
        .expect("CASP should validate");
}

fn cfvt_instruction(mnemonic: &str) -> Instruction {
    casp_instruction(mnemonic)
}