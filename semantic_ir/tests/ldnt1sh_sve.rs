/// LDNT1SH (scalar plus scalar) - Contiguous load non-temporal signed halfwords
/// Stanford source: arm64_md_clean/ldnt1sh_z_p_ar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    types::Reg,
    validate_instruction,
};

fn ldnt1sh_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let msize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 };
    let base = Expr::ReadScalableVecReg { reg: 1, vl };
    let offset = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::NonTemporalLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(offset),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: false,
        first_fault: false,
        non_fault: false,
        non_temporal: true,
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDNT1SH (scalar plus scalar)".to_string(),
            encoding: Some("Contiguous load non-temporal signed halfwords".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnt1sh_z_p_ar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnt1sh_sve_validates() {
    let instruction = ldnt1sh_sve_instruction();
    validate_instruction(&instruction).expect("LDNT1SH SVE should validate");
}
