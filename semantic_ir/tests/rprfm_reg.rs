/// RPRFM (register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rprfm_reg.md
///
/// Range prefetch hint: MemRead of the base address + Discard (hint has no
/// register writeback), same precedent as PRFM.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    validate_instruction,
};

fn rprfm_reg_instruction() -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 64, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };
    let data = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits64,
    };
    let semantics = vec![Stmt::Discard { value: data }];

    Instruction {
        id: InstructionId {
            mnemonic: "RPRFM".to_string(),
            encoding: Some("Range prefetch memory (register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_RPRFM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rprfm_reg.md".to_string(),
            source_sha256: "t".to_string(),
            clean_sha256: "t".to_string(),
        },
    }
}

#[test]
fn rprfm_reg_validates() {
    validate_instruction(&rprfm_reg_instruction()).expect("RPRFM should validate");
}

#[test]
fn rprfm_reg_reads_memory() {
    let instruction = rprfm_reg_instruction();
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Discard { value: Expr::MemRead { .. } }
    ));
}
