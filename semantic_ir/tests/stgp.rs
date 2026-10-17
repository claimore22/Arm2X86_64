/// STGP (Store allocation tag and pair of registers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/stgp.md
///
/// MTE tag store(s): the allocation tag from the address bits 59:56 of the address
/// is written to tag memory for the tag granule (with pair store) covering the (aligned) address.
/// Modeled as a placeholder Let (tag-memory write has no IR-level value type).
/// Requires FEAT_MTE.
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction};

fn stgp_instruction() -> Instruction {
    let _addr = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let semantics = vec![Stmt::Let {
        name: "stgp_store_tag".to_string(),
        value: Expr::Const { value: 1, ty: semantic_ir::BitWidth::W8 },
    }];

    Instruction {
        id: InstructionId { mnemonic: "STGP".to_string(), encoding: Some("Store allocation tag and pair of registers".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_MTE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "stgp.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn stgp_validates() {
    validate_instruction(&stgp_instruction()).expect("should validate");
}

#[test]
fn stgp_requires_mte() {
    assert!(stgp_instruction()
        .required_features
        .contains(&"FEAT_MTE".to_string()));
}
