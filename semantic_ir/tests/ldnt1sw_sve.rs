/// LDNT1SW (scalar plus scalar) - Contiguous load non-temporal signed words
/// Stanford source: arm64_md_clean/ldnt1sw_z_p_ar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn ldnt1sw_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Z[n] - base
    let offset = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 }; // X[m] - scalar offset
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Z[t] - destination

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ldnt1hScalar,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(base), Box::new(offset)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1SW (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous load non-temporal signed words to vector (scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnt1sw_z_p_ar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnt1sw_sve_scalar_validates() {
    let instruction = ldnt1sw_sve_scalar_instruction();
    validate_instruction(&instruction).expect("LDNT1SW SVE scalar should validate");
}
