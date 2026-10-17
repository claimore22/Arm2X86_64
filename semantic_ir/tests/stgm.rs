/// STGM (Store tag multiple) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stgm.md
///
/// MTE tag store(s): the allocation tag from the address bits 59:56 from X(s)
/// is written to tag memory for the block of tag granules covering the (aligned) address.
/// Modeled as a placeholder Let (tag-memory write has no IR-level value type).
/// Requires FEAT_MTE2.
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn stgm_instruction() -> Instruction {
    let _addr = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let semantics = vec![Stmt::Let {
        name: "stgm_store_tag".to_string(),
        value: Expr::Const { value: 1, ty: semantic_ir::BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "STGM".to_string(), encoding: Some("Store tag multiple".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stgm.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stgm_validates() {
    validate_instruction(&stgm_instruction()).expect("should validate");
}

#[test]
fn stgm_requires_mte() {
    assert!(stgm_instruction()
        .required_features
        .contains(&"FEAT_MTE2".to_string()));
}
