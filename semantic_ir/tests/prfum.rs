/// PRFUM (Prefetch memory, unscaled offset) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/prfum.md
///
/// Prefetch is a hint: modeled as MemRead of the target address plus Discard
/// (no register writeback). Unscaled signed offset.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn prfum_instruction() -> Instruction {
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };

    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };

    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits64,
    };

    let semantics = vec![Stmt::Discard { value: data }];

    Instruction {
        id: InstructionId {
            mnemonic: "PRFUM".to_string(),
            encoding: Some("PRFUM (Prefetch memory, unscaled offset)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "prfum.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn prfum_validates() {
    validate_instruction(&prfum_instruction()).expect("PRFUM should validate");
}

#[test]
fn prfum_reads_memory() {
    let instruction = prfum_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard {
            value: Expr::MemRead { .. }
        }
    ));
}
