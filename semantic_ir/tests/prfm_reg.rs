/// PRFM (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/prfm_reg.md
///
/// Prefetch is a hint: modeled as MemRead of the target address plus Discard
/// (no register writeback). Register offset prefetch.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn prfm_reg_instruction() -> Instruction {
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
            mnemonic: "PRFM".to_string(),
            encoding: Some("PRFM (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "prfm_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn prfm_reg_validates() {
    validate_instruction(&prfm_reg_instruction()).expect("PRFM should validate");
}

#[test]
fn prfm_reg_reads_memory() {
    let instruction = prfm_reg_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard {
            value: Expr::MemRead { .. }
        }
    ));
}
