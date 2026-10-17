/// LD1D (scalar plus vector) - Gather load doublewords to vector (vector index)
/// Stanford source: arm64_md_clean/ld1d_z_p_bz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ld1d_sve_gather_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W64;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadScalableVecReg { reg: 1, vl };
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
        scale: 3,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1D".to_string(),
            encoding: Some("Gather load doublewords to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1d_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1d_sve_gather_validates() {
    let instruction = ld1d_sve_gather_instruction();
    validate_instruction(&instruction).expect("LD1D SVE gather should validate");
}

#[test]
fn ld1d_sve_gather_uses_sve_gather_scatter() {
    let instruction = ld1d_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, .. } = value else { panic!("expected SVEGatherScatter for LD1D"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W64);
}