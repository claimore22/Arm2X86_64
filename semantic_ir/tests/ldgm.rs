/// LDGM (Load Tag Multiple) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldgm.md
/// Load multiple allocation tags. Requires FEAT_MTE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldgm_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let tag0 = Expr::MemTagRead {
        address: Box::new(address.clone()),
    };
    let tag1 = Expr::MemTagRead {
        address: Box::new(Expr::Binary {
            op: semantic_ir::BinOp::Add,
            left: Box::new(address.clone()),
            right: Box::new(Expr::Const {
                value: 16,
                ty: BitWidth::W64,
            }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: Expr::ZeroExtend {
                value: Box::new(tag0),
                to: BitWidth::W64,
            },
        },
        Stmt::WriteReg {
            reg: Reg::X(3),
            value: Expr::ZeroExtend {
                value: Box::new(tag1),
                to: BitWidth::W64,
            },
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Tag Multiple".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldgm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldgm_validates() {
    validate_instruction(&ldgm_instruction("LDGM")).expect("LDGM should validate");
}
