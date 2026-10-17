/// PRFH (scalar plus immediate) - Contiguous prefetch halfwords
/// Stanford source: arm64_md_clean/prfh_mz_p_bi.md
/// PRFH (scalar plus scalar) - Contiguous prefetch halfwords
/// Stanford source: arm64_md_clean/prfh_mz_p_br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn prfh_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }; // Zt - destination

    let offset = Expr::Const { value: 0, ty: BitWidth::W4 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfhImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(offset)],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl: BitWidth::W256 }];

    Instruction {
        id: InstructionId {
            mnemonic: "PRFH (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous prefetch halfwords (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: "prfh_mz_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn prfh_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let offset = Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let destination = Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }; // Zt - destination

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::PrfhScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }), Box::new(Expr::ReadReg { reg: semantic_ir::types::Reg::X(2), width: BitWidth::W64 })],
        vl: BitWidth::W256,
        esize: None,
        imm: None,
    };

    Instruction {
        id: InstructionId {
            mnemonic: "PRFH (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous prefetch halfwords (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl: BitWidth::W256 }],
        provenance: Provenance {
            source_path: "prfh_mz_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn prfh_sve_imm_validates() {
    let instruction = prfh_sve_imm_instruction();
    validate_instruction(&instruction).expect("PRFH imm SVE should validate");
}

#[test]
fn prfh_sve_scalar_validates() {
    let instruction = prfh_sve_scalar_instruction();
    validate_instruction(&instruction).expect("PRFH scalar SVE should validate");
}