/// LDXP (Load Exclusive Pair) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldxp.md
/// Load exclusive pair. Requires FEAT_LSE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldxp_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let addr2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(address.clone()),
        right: Box::new(Expr::Const {
            value: 8,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    let data0 = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits64,
    };
    let data1 = Expr::MemRead {
        address: Box::new(addr2),
        size: MemSize::Bits64,
    };
    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: data0,
        },
        Stmt::WriteReg {
            reg: Reg::X(3),
            value: data1,
        },
    ];
    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Exclusive Pair".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldxp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}
#[test]
fn ldxp_validates() {
    validate_instruction(&cfvt_instruction("LDXP"))
        .expect("LDXP should validate");
}
fn cfvt_instruction(mnemonic: &str) -> Instruction {
    ldxp_instruction(mnemonic)
}