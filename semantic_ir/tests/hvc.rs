/// HVC (Hypervisor call) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/hvc.md
///
/// AArch64_CallHypervisor(imm16): opaque exception to EL2. Modeled as a placeholder Let statement, same precedent as CSDB/CLRBHB.
use semantic_ir::{Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn hvc_instruction() -> Instruction {
    let semantics = vec![Stmt::Let {
        name: "hvc".to_string(),
        value: Expr::Const {
            value: 1,
            ty: semantic_ir::BitWidth::W8,
        },
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HVC".to_string(),
            encoding: Some("Hypervisor call".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "hvc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn hvc_validates() {
    let instruction = hvc_instruction();
    validate_instruction(&instruction).expect("HVC should validate");
}

#[test]
fn hvc_is_placeholder_operation() {
    let instruction = hvc_instruction();

    assert_eq!(instruction.semantics.len(), 1);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "hvc"
    ));
}
