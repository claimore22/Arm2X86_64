/// FIRSTP - Scalar index of first true predicate element (predicated)
/// Stanford source: arm64_md_clean/firstp_r_p_p.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn firstp_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadPredReg { reg: 1, pl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FirstP,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FIRSTP".to_string(),
            encoding: Some("Scalar index of first true predicate element (predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p2".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "firstp_r_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn firstp_sve_validates() {
    let instruction = firstp_sve_instruction();
    validate_instruction(&instruction).expect("FIRSTP SVE should validate");
}