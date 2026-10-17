/// LD3H (scalar plus scalar) - Contiguous load three-halfword structures to three vectors
/// Stanford source: arm64_md_clean/ld3h_z_p_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld3h_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W16;
    let msize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 };
    let index = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(destination),
        vl, esize, msize, scale: 0,
        offs_unsigned: true, data_unsigned: true,
        first_fault: false, non_fault: false, non_temporal: false,
        contiguous: true, tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId { mnemonic: "LD3H".to_string(), encoding: Some("Contiguous load three-halfword structures to three vectors".to_string()) },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance { source_path: "ld3h_z_p_zz.md".to_string(), source_sha256: "test-source-sha256".to_string(), clean_sha256: "test-clean-sha256".to_string() },
    }
}

#[test]
fn ld3h_sve_validates() {
    let instruction = ld3h_sve_instruction();
    validate_instruction(&instruction).expect("LD3H SVE should validate");
}
