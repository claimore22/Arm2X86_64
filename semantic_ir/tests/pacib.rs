/// PACIB, PACIB1716, PACIBSP, PACIBZ, PACIZB (PAC for instruction address, key B) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacib.md
///
/// Computes and inserts a Pointer Authentication Code using key I B.
/// Modeled with the opaque `Expr::AddPAC` node (QARMA cipher not composed in IR).
/// Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, Stmt,
    validate_instruction,
};

fn pacib_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let modifier = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let result = Expr::AddPAC {
        kind: PacKind::IB,
        address: Box::new(address),
        modifier: Box::new(modifier),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PACIB".to_string(),
            encoding: Some("PACIB, PACIB1716, PACIBSP, PACIBZ, PACIZB (PAC for instruction address, key B)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacib.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacib_validates() {
    validate_instruction(&pacib_instruction()).expect("PACIB should validate");
}

#[test]
fn pacib_requires_pauth() {
    assert!(pacib_instruction()
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}

#[test]
fn pacib_uses_addpac_key() {
    let instruction = pacib_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::AddPAC { kind, .. } = value else {
        panic!("expected AddPAC");
    };
    assert_eq!(*kind, PacKind::IB);
}
