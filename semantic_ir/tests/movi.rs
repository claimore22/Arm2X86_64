/// MOVI: Move immediate (vector)
/// Stanford source: movi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn movi_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    // MOVI: Move immediate value to vector
    let result = Expr::Const {
        value: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128, // Example immediate
        ty: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOVI".to_string(),
            encoding: Some("Move immediate (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "movi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn movi_validates() {
    let instruction = movi_instruction();
    validate_instruction(&instruction).expect("MOVI should validate");
}

#[test]
fn movi_uses_const() {
    let instruction = movi_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Const { ty, .. } = value else {
        panic!("expected Const for MOVI");
    };

    assert_eq!(*ty, BitWidth::W128);
}