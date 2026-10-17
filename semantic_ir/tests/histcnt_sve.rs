/// HISTCNT - Count matching elements in vector (predicated)
/// Stanford source: arm64_md_clean/histcnt_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn histcnt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Histcnt,
        predicate: Some(Box::new(predicate)),
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
            mnemonic: "HISTCNT".to_string(),
            encoding: Some("Count matching elements in vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "histcnt_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn histcnt_sve_validates() {
    let instruction = histcnt_sve_instruction();
    validate_instruction(&instruction).expect("HISTCNT SVE should validate");
}