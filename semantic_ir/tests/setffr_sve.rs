/// SETFFR - Initialize the first-fault register to all true
/// Stanford source: arm64_md_clean/setffr_f.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn setffr_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    // SETFFR: FFR = all true (predicate mask of all ones)
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Setffr,
        predicate: None,
        operands: vec![],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "SETFFR".to_string(),
            encoding: Some("Initialize the first-fault register to all true".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "setffr_f.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn setffr_sve_validates() {
    let instruction = setffr_sve_instruction();
    validate_instruction(&instruction).expect("SETFFR SVE should validate");
}

#[test]
fn setffr_sve_structure() {
    let instruction = setffr_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, .. } = value else {
        panic!("expected SVESpecial for SETFFR");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Setffr);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 0);
    assert_eq!(*vl, BitWidth::W256);
}