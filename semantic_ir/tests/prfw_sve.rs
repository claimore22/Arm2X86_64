/// PRFW (scalar plus immediate) - Contiguous prefetch words
/// Stanford source: arm64_md_clean/prfw_mz_p_bi.md
/// PRFW (scalar plus scalar) - Contiguous prefetch words
/// Stanford source: arm64_md_clean/prfw_mz_p_br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn prfw_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }; // Zt - destination

    let offset = Expr::Const { value: 0, ty: BitWidth::W4 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfwImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(offset)],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 }];

    Instruction {
        id: InstructionId {
            mnemonic: "PRFW (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous prefetch words (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: "prfw_mz_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn prfw_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let offset = Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }; // Zt - destination

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfwScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 })],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    Instruction {
        id: InstructionId {
            mnemonic: "PRFW (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous prefetch words (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: "prfw_mz_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn prfw_sve_imm_validates() {
    let instruction = prfw_sve_imm_instruction();
    validate_instruction(&instruction).expect("PRFW imm SVE should validate");
}

#[test]
fn prfw_sve_scalar_validates() {
    let instruction = prfw_sve_scalar_instruction();
    validate_instruction(&instruction).expect("PRFW scalar SVE should validate");
}