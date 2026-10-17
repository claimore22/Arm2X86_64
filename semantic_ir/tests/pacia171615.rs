/// PACIA171615 (PAC for instruction address, key A, second modifier) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/pacia171615.md
///
/// Two-modifier PAC for return addresses (FEAT_PAuth_LR, PSTATE.PACM == 1):
/// modeled with the opaque `Expr::AddPAC2` node (key IA).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, PacKind, Provenance, Reg, SpecialReg, Stmt,
    validate_instruction,
};

fn pacia171615_instruction() -> Instruction {
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
            mnemonic: "PACIA171615".to_string(),
            encoding: Some("PACIA171615 (PAC for instruction address, key A, second modifier)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth_LR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "pacia171615.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn pacia171615_validates() {
    validate_instruction(&pacia171615_instruction()).expect("PACIA171615 should validate");
}

#[test]
fn pacia171615_requires_pauth_lr() {
    assert!(pacia171615_instruction()
        .required_features
        .contains(&"FEAT_PAuth_LR".to_string()));
}

#[test]
fn pacia171615_uses_addpac2() {
    let instruction = pacia171615_instruction();
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
