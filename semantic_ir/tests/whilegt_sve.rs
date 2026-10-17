/// WHILEGT (predicate) - While decrementing signed scalar greater than scalar
/// Stanford source: arm64_md_clean/whilegt_p_p_rr.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn whilegt_sve_instruction() -> Instruction {
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let operand1 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 }; // Rn
    let operand2 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rm

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Whilegt,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl: BitWidth::W256,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![
        Stmt::WritePredReg { reg: 0, value: result, pl },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "WHILEGT".to_string(),
            encoding: Some("While decrementing signed scalar greater than scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "whilegt_p_p_rr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn whilegt_sve_validates() {
    let instruction = whilegt_sve_instruction();
    validate_instruction(&instruction).expect("WHILEGT SVE should validate");
}

#[test]
fn whilegt_sve_structure() {
    let instruction = whilegt_sve_instruction();

    let Stmt::WritePredReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WritePredReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for WHILEGT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Whilegt);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 });
    matches!(operands[1].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W8);
}