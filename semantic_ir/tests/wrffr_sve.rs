/// WRFFR - Write the first-fault register
/// Stanford source: arm64_md_clean/wrffr_f_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn wrffr_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Wrffr,
        predicate: None,
        operands: vec![Box::new(predicate)],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WritePredReg { reg: 0, value: result, pl }];

    Instruction {
        id: InstructionId {
            mnemonic: "WRFFR".to_string(),
            encoding: Some("Write the first-fault register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "wrffr_f_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn wrffr_sve_validates() {
    let instruction = wrffr_sve_instruction();
    validate_instruction(&instruction).expect("WRFFR SVE should validate");
}

#[test]
fn wrffr_sve_structure() {
    let instruction = wrffr_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, .. } = value else {
        panic!("expected SVESpecial for WRFFR");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Wrffr);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(*vl, BitWidth::W256);
}