/// FCPY (SVE predicated) - Copy floating-point immediate to vector elements
/// Stanford source: arm64_md_clean/fcpy_z_p_i.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   var result : bits(VL) = Z[](d);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           result[e*esize] = imm;
///       end;
///   end;
///   Z[](d) = result;
///
/// Copies a floating-point immediate (expanded from imm8) into each active element of Z[d].
/// Inactive elements remain unmodified (merging predication).
/// This instruction is also known as FMOV (immediate, predicated).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn fcpy_sve_merging_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let dest = Expr::ReadScalableVecReg { reg: 0, vl };

    // Floating-point immediate: FPOne (1.0) for esize=32
    // This represents the expanded VFPExpandImm(imm8)
    let fp_imm = Expr::Const {
        value: 0x3f800000u128, // 1.0 in IEEE 754 single-precision
        ty: BitWidth::W32,
    };

    // First, broadcast the FP immediate to all elements
    let broadcast_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(fp_imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    // Then copy the broadcast immediate to destination with merging predication
    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id,
        predicate: Box::new(predicate),
        operand: Box::new(broadcast_imm),
        destination: Box::new(dest),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCPY".to_string(),
            encoding: Some("Copy floating-point immediate to vector elements (merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcpy_z_p_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fcpy_sve_zeroing_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let dest = Expr::ReadScalableVecReg { reg: 0, vl };

    // Floating-point immediate: FPOne (1.0)
    let fp_imm = Expr::Const {
        value: 0x3f800000u128, // 1.0 in IEEE 754 single-precision
        ty: BitWidth::W32,
    };

    let broadcast_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(fp_imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id,
        predicate: Box::new(predicate),
        operand: Box::new(broadcast_imm),
        destination: Box::new(dest),
        vl,
        esize,
        predication: PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FCPY".to_string(),
            encoding: Some("Copy floating-point immediate to vector elements (zeroing)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fcpy_z_p_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fcpy_sve_merging_validates() {
    let instruction = fcpy_sve_merging_instruction();
    validate_instruction(&instruction).expect("FCPY SVE merging should validate");
}

#[test]
fn fcpy_sve_merging_structure() {
    let instruction = fcpy_sve_merging_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for FCPY merging");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Id);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Merging);
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    // Operand should be SVESpecial with Dup
    let Expr::SVESpecial { op: sv_op, operands, .. } = operand.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
}

#[test]
fn fcpy_sve_zeroing_validates() {
    let instruction = fcpy_sve_zeroing_instruction();
    validate_instruction(&instruction).expect("FCPY SVE zeroing should validate");
}

#[test]
fn fcpy_sve_zeroing_structure() {
    let instruction = fcpy_sve_zeroing_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap {
        op,
        predicate,
        operand,
        destination,
        vl,
        esize,
        predication,
    } = value else {
        panic!("expected ScalableVectorMap for FCPY zeroing");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorUnaryOp::Id);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, PredicationMode::Zeroing);
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    let Expr::SVESpecial { op: sv_op, operands, .. } = operand.as_ref() else {
        panic!("expected SVESpecial for broadcast immediate");
    };
    assert_eq!(*sv_op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
}