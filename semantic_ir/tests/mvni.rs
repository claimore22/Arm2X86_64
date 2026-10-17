/// MVNI: Move inverted immediate (vector)
/// Stanford source: mvni.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn mvni_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    // MVNI: Move inverted immediate
    let result = Expr::Const {
        value: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128, // Inverted immediate
        ty: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MVNI".to_string(),
            encoding: Some("Move inverted immediate (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mvni.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mvni_validates() {
    let instruction = mvni_instruction();
    validate_instruction(&instruction).expect("MVNI should validate");
}

#[test]
fn mvni_uses_const() {
    let instruction = mvni_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Const { ty, .. } = value else {
        panic!("expected Const for MVNI");
    };

    assert_eq!(*ty, BitWidth::W128);
}