/// PACIASPPC (PAC for return address, key A) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/paciasppc.md
///
/// Two-modifier PAC for return addresses (FEAT_PAuth_LR, PSTATE.PACM == 1):
/// modeled with the opaque `Expr::AddPAC2` node (key IA).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn paciasppc_instruction() -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(30),
        width: BitWidth::W64,
    };
    let modifier1 = Expr::ReadSP {
        width: BitWidth::W64,
    };
    let modifier2 = Expr::ReadSpecial {
        reg: SpecialReg::Pc64,
        width: BitWidth::W64,
    };

    let result = Expr::AddPAC2 {
        kind: PacKind::IA,
        address: Box::new(address),
        modifier1: Box::new(modifier1),
        modifier2: Box::new(modifier2),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(30),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "PACIASPPC".to_string(),
            encoding: Some("PACIASPPC (PAC for return address, key A)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "paciasppc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn paciasppc_validates() {
    validate_instruction(&paciasppc_instruction()).expect("PACIASPPC should validate");
}

#[test]
fn paciasppc_requires_pauth_lr() {
    assert!(paciasppc_instruction()
        .required_features
        .contains(&"FEAT_PAuth_LR".to_string()));
}

#[test]
fn paciasppc_uses_addpac2() {
    let instruction = paciasppc_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(
        value,
        Expr::AddPAC2 {
            kind: PacKind::IA,
..
        }
    ));
}
