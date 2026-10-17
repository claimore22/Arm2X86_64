/// PACGA (PAC using generic key) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacga.md
///
/// Computes and inserts a Pointer Authentication Code using key G A.
/// Modeled with the opaque `Expr::AddPAC` node (QARMA cipher not composed in IR).
/// Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, Stmt,
    validate_instruction,
};

fn pacga_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let result = Expr::AddPAC {
        kind: PacKind::GA,
        address: Box::new(address),
        modifier: Box::new(modifier),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PACGA".to_string(),
            encoding: Some("PACGA (PAC using generic key)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacga.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacga_validates() {
    validate_instruction(&pacga_instruction()).expect("PACGA should validate");
}

#[test]
fn pacga_requires_pauth() {
    assert!(pacga_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}

#[test]
fn pacga_uses_addpac_key() {
    let instruction = pacga_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::AddPAC { kind, .. } = value else {
        panic!("expected AddPAC");
    };
    assert_eq!(*kind, PacKind::GA);
}
