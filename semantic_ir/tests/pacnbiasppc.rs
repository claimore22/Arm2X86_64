/// PACNBIASPPC (PAC for return address, key A, not a branch target) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacnbiasppc.md
///
/// Two-modifier PAC for return addresses (FEAT_PAuth_LR, PSTATE.PACM == 1):
/// modeled with the opaque `Expr::AddPAC2` node (key IA).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn pacnbiasppc_instruction() -> Instruction {
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
            mnemonic: "PACNBIASPPC".to_string(),
            encoding: Some("PACNBIASPPC (PAC for return address, key A, not a branch target)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacnbiasppc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacnbiasppc_validates() {
    validate_instruction(&pacnbiasppc_instruction()).expect("PACNBIASPPC should validate");
}

#[test]
fn pacnbiasppc_requires_pauth_lr() {
    assert!(pacnbiasppc_instruction()
        .required_features
        .contains(&"FEAT_PAuth_LR".to_string()));
}

#[test]
fn pacnbiasppc_uses_addpac2() {
    let instruction = pacnbiasppc_instruction();
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
