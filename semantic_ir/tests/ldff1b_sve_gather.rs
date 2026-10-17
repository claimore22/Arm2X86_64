/// LDFF1B (scalar plus vector) - Gather load first-fault unsigned bytes to vector (vector index)
/// Stanford source: arm64_md_clean/ldff1b_z_p_bz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ldff1b_sve_gather_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W8;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad,
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
        first_fault: true,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LDFF1B".to_string(),
            encoding: Some("Gather load first-fault unsigned bytes to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldff1b_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldff1b_sve_gather_validates() {
    let instruction = ldff1b_sve_gather_instruction();
    validate_instruction(&instruction).expect("LDFF1B SVE first-fault gather should validate");
}

#[test]
fn ldff1b_sve_gather_uses_sve_gather_scatter() {
    let instruction = ldff1b_sve_gather_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, first_fault, .. } = value else { panic!("expected SVEGatherScatter for LDFF1B"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::FirstFaultLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*first_fault, true);
}