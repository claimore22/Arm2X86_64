/// PRFB (scalar plus immediate) - Contiguous prefetch bytes
/// Stanford source: arm64_md_clean/prfb_mz_p_bi.md
/// PRFB (scalar plus scalar) - Contiguous prefetch bytes
/// Stanford source: arm64_md_clean/prfb_mz_p_br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn prfb_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    let offset = Expr::Const { value: 0, ty: BitWidth::W4 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfbImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(offset)],
        vl,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "PRFB (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous prefetch bytes (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "prfb_mz_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn prfb_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let offset = Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - destination

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfbScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 })],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    Instruction {
        id: InstructionId {
            mnemonic: "PRFB (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous prefetch bytes (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "prfb_mz_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn prfb_sve_imm_validates() {
    let instruction = prfb_sve_imm_instruction();
    validate_instruction(&instruction).expect("PRFB imm SVE should validate");
}

#[test]
fn prfb_sve_scalar_validates() {
    let instruction = prfb_sve_scalar_instruction();
    validate_instruction(&instruction).expect("PRFB scalar SVE should validate");
}