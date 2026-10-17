/// PACNBIBSPPC (PAC for return address, key B, not a branch target) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacnbibsppc.md
///
/// Two-modifier PAC for return addresses (FEAT_PAuth_LR, PSTATE.PACM == 1):
/// modeled with the opaque `Expr::AddPAC2` node (key IB).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn pacnbibsppc_instruction() -> Instruction {
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
        kind: PacKind::IB,
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
            mnemonic: "PACNBIBSPPC".to_string(),
            encoding: Some("PACNBIBSPPC (PAC for return address, key B, not a branch target)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacnbibsppc.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacnbibsppc_validates() {
    validate_instruction(&pacnbibsppc_instruction()).expect("PACNBIBSPPC should validate");
}

#[test]
fn pacnbibsppc_requires_pauth_lr() {
    assert!(pacnbibsppc_instruction()
        .required_features
        .contains(&"FEAT_PAuth_LR".to_string()));
}

#[test]
fn pacnbibsppc_uses_addpac2() {
    let instruction = pacnbibsppc_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(
        value,
        Expr::AddPAC2 {
            kind: PacKind::IB,
..
        }
    ));
}
