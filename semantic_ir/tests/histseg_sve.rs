/// HISTSEG - Count matching elements in vector segments (unpredicated)
/// Stanford source: arm64_md_clean/histseg_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn histseg_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W8;

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Histseg,
        predicate: None,
        operands: vec![
            Box::new(operand1),
            Box::new(operand2),
            Box::new(destination),
        ],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "HISTSEG".to_string(),
            encoding: Some("Count matching elements in vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "histseg_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn histseg_sve_validates() {
    let instruction = histseg_sve_instruction();
    validate_instruction(&instruction).expect("HISTSEG SVE should validate");
}