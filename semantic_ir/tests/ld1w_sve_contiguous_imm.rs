/// LD1W (scalar plus immediate, single register) - Contiguous load unsigned words to vector (immediate index)
/// Stanford source: arm64_md_clean/ld1w_z_p_bi.md (similar to LD1H)
///
/// For words, msize = 32, mbytes = 4.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1w_sve_contiguous_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W32; // Memory access size (32-bit words)

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    let base = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    let imm_index = Expr::Const {
        value: 3i32 as u128,
        ty: BitWidth::W4,
    };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_index),
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
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1W".to_string(),
            encoding: Some("Contiguous load unsigned words to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1w_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1w_sve_contiguous_imm_validates() {
    let instruction = ld1w_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD1W contiguous immediate should validate");
}

#[test]
fn ld1w_sve_contiguous_imm_structure() {
    let instruction = ld1w_sve_contiguous_imm_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
        panic!("expected SVEGatherScatter for LD1W contiguous immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W32);
    assert_eq!(*contiguous, true);
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::Const { value: 3, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}