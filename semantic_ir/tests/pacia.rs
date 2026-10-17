/// PACIA, PACIA1716, PACIASP, PACIAZ, PACIZA (PAC for instruction address, key A) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacia.md
///
/// Computes and inserts a Pointer Authentication Code using key I A.
/// Modeled with the opaque `Expr::AddPAC` node (QARMA cipher not composed in IR).
/// Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, Stmt,
    validate_instruction,
};

fn pacia_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let result = Expr::AddPAC {
        kind: PacKind::IA,
        address: Box::new(address),
        modifier: Box::new(modifier),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PACIA".to_string(),
            encoding: Some("PACIA, PACIA1716, PACIASP, PACIAZ, PACIZA (PAC for instruction address, key A)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacia.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacia_validates() {
    validate_instruction(&pacia_instruction()).expect("PACIA should validate");
}

#[test]
fn pacia_requires_pauth() {
    assert!(pacia_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}

#[test]
fn pacia_uses_addpac_key() {
    let instruction = pacia_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::AddPAC { kind, .. } = value else {
        panic!("expected AddPAC");
    };
    assert_eq!(*kind, PacKind::IA);
}
