/// LDNF1D (Load Non-Fault Multiple Doubleword) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldnf1d.md
/// Non-faulting load of multiple doublewords. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};
use semantic_ir::primitives::SVEGatherScatterOp;

fn ldnf1d_instruction(mnemonic: &str) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let index = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W64,
    };

    let data = Expr::SVEGatherScatter {
        op: SVEGatherScatterOp::NonFaultLoad,
        predicate: Box::new(Expr::ReadPredReg {
            reg: 0,
            pl: BitWidth::W16,
        }),
        base: Box::new(address),
        index: Box::new(index),
        destination: Box::new(Expr::ReadScalableVecReg {
            reg: 0,
            vl: BitWidth::W128,
        }),
        vl: BitWidth::W128,
        esize: BitWidth::W64,
        msize: BitWidth::W64,
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
            encoding: Some("Load Non-Fault Multiple Doubleword".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldnf1d.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldnf1d_validates() {
    validate_instruction(&ldnf1d_instruction("LDNF1D")).expect("LDNF1D should validate");
}

#[test]
fn ldnf1h_validates() {
    validate_instruction(&ldnf1d_instruction("LDNF1H")).expect("LDNF1H should validate");
}

#[test]
fn ldnf1w_validates() {
    validate_instruction(&ldnf1d_instruction("LDNF1W")).expect("LDNF1W should validate");
}
