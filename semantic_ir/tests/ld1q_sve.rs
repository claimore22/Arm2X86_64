/// LD1Q - Gather load quadwords
/// Stanford source: arm64_md_clean/ld1q_z_p_ar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1q_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W128;
    let msize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadScalableVecReg { reg: 1, vl };
    let index = Expr::ReadReg { reg: Reg::X(2), width: BitWidth::W64 };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1Q".to_string(),
            encoding: Some("Gather load quadwords".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1q_z_p_ar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1q_sve_validates() {
    let instruction = ld1q_sve_instruction();
    validate_instruction(&instruction).expect("LD1Q SVE should validate");
}
