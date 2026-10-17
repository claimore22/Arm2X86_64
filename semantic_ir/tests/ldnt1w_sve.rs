/// LDNT1W (scalar plus immediate, single register) - Contiguous load non-temporal words
/// Stanford source: arm64_md_clean/ldnt1w_z_p_bi.md
/// LDNT1W (scalar plus scalar, single register) - Contiguous load non-temporal words
/// Stanford source: arm64_md_clean/ldnt1w_z_p_br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn ldnt1w_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    let offset = Expr::Const { value: 0, ty: BitWidth::W4 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ldnt1wImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(offset)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1W (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous load non-temporal words to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "ldnt1w_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ldnt1w_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let offset = Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ldnt1wScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 })],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1W (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous load non-temporal words to vector (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "ldnt1w_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnt1w_sve_imm_validates() {
    let instruction = ldnt1w_sve_imm_instruction();
    validate_instruction(&instruction).expect("LDNT1W imm SVE should validate");
}

#[test]
fn ldnt1w_sve_scalar_validates() {
    let instruction = ldnt1w_sve_scalar_instruction();
    validate_instruction(&instruction).expect("LDNT1W scalar SVE should validate");
}