/// INSR (SVE scalar) - Insert general-purpose register in shifted vector
/// Stanford source: arm64_md_clean/insr_z_r.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn insr_sve_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - destination
    let source = Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 }; // Rm - general-purpose register

    // INSR: shift vector left by one element, insert scalar at element 0
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Insr,
        predicate: None,
        operands: vec![Box::new(destination), Box::new(source)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "INSR (scalar)".to_string(),
            encoding: Some("Insert general-purpose register in shifted vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "insr_z_r.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn insr_sve_fp_scalar_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zdn - destination
    let source = Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W64 }; // Vm - SIMD&FP scalar register

    // INSR: shift vector left by one element, insert SIMD&FP scalar at element 0
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Insr,
        predicate: None,
        operands: vec![Box::new(destination), Box::new(source)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "INSR (SIMD&FP scalar)".to_string(),
            encoding: Some("Insert SIMD&FP scalar register in shifted vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "insr_z_v.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn insr_sve_scalar_validates() {
    let instruction = insr_sve_scalar_instruction();
    validate_instruction(&instruction).expect("INSR SVE scalar should validate");
}

#[test]
fn insr_sve_fp_scalar_validates() {
    let instruction = insr_sve_fp_scalar_instruction();
    validate_instruction(&instruction).expect("INSR SVE SIMD&FP scalar should validate");
}

#[test]
fn insr_sve_scalar_structure() {
    let instruction = insr_sve_scalar_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for INSR");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Insr);
    assert!(predicate.is_none());
    assert_eq!(operands.len(), 2);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W64);
    assert!(imm.is_none());
}