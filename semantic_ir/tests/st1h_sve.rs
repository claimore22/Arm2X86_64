/// ST1H (scalar plus immediate, single register) - Contiguous store halfwords from vector
/// Stanford source: arm64_md_clean/st1h_z_p_bi.md
/// ST1H (scalar plus scalar, single register) - Contiguous store halfwords from vector
/// Stanford source: arm64_md_clean/st1h_z_p_br.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn st1h_sve_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let src = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - source vector

    let offset = Expr::Const { value: 0, ty: BitWidth::W4 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::St1hImm,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }), Box::new(src), Box::new(offset)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result.clone(), vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ST1H (scalar plus immediate)".to_string(),
            encoding: Some("Contiguous store halfwords from vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "st1h_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn st1h_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Rn - base
    let index = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }; // Rm - scalar index
    let src = Expr::ReadScalableVecReg { reg: 0, vl }; // Zt - source vector

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::St1hScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }), Box::new(Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }), Box::new(src)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    Instruction {
        id: InstructionId {
            mnemonic: "ST1H (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous store halfwords from vector (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }],
        provenance: Provenance {
            source_path: "st1h_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn st1h_sve_imm_validates() {
    let instruction = st1h_sve_imm_instruction();
    validate_instruction(&instruction).expect("ST1H imm SVE should validate");
}

#[test]
fn st1h_sve_scalar_validates() {
    let instruction = st1h_sve_scalar_instruction();
    validate_instruction(&instruction).expect("ST1H scalar SVE should validate");
}