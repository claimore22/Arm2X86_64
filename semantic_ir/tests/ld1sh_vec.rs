/// LD1SH (Load Single Halfword, scalar plus vector) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ld1sh.md
/// Load single halfword with scalar plus vector addressing. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ld1sh_vec_instruction(mnemonic: &str) -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let index = Expr::ReadScalableVecReg {
        reg: 0,
        vl: BitWidth::W128,
    };

    let data = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::NonFaultLoad,
        predicate: Box::new(Expr::ReadPredReg {
            reg: 0,
            pl: BitWidth::W16,
        }),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg {
            reg: 0,
            vl: BitWidth::W128,
        }),
        vl: BitWidth::W128,
        esize: BitWidth::W16,
        msize: BitWidth::W16,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: true,
        non_temporal: false,
        contiguous: true,
        tag_checked: false,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: data,
        vl: BitWidth::W128,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load Single Halfword Scalar plus Vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1sh_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1sh_vec_validates() {
    validate_instruction(&ld1sh_vec_instruction("LD1SH"))
        .expect("LD1SH scalar plus vector should validate");
}
