/// LDNP (Load Pair of Registers, Non-temporal) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldnp.md
/// Load pair of registers non-temporal.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldnp_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();
    let dbytes = (size.width().bits() / 8) as u128;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let addr2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base.clone()),
        right: Box::new(Expr::Const {
            value: dbytes,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let data0 = Expr::MemRead {
        address: Box::new(base),
        size,
    };
    let data1 = Expr::MemRead {
        address: Box::new(addr2),
        size,
    };

    let v0 = if width != BitWidth::W64 {
        Expr::ZeroExtend {
            value: Box::new(data0),
            to: BitWidth::W64,
        }
    } else {
        data0
    };
    let v1 = if width != BitWidth::W64 {
        Expr::ZeroExtend {
            value: Box::new(data1),
            to: BitWidth::W64,
        }
    } else {
        data1
    };

    let semantics = vec![
        Stmt::WriteReg { reg: Reg::X(2), value: v0 },
        Stmt::WriteReg { reg: Reg::X(4), value: v1 },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Pair of Registers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnp_validates() {
    validate_instruction(&ldnp_instruction("LDNP", MemSize::Bits64)).expect("LDNP should validate");
}

#[test]
fn ldp_validates() {
    validate_instruction(&ldnp_instruction("LDP", MemSize::Bits64)).expect("LDP should validate");
}

#[test]
fn ldpsw_validates() {
    validate_instruction(&ldnp_instruction("LDPSW", MemSize::Bits32))
        .expect("LDPSW should validate");
}
