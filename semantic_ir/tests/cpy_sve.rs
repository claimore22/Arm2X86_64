/// CPY (immediate, merging) - Copy signed integer immediate to vector elements (merging)
/// Stanford source: arm64_md_clean/cpy_z_p_i.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);
///   let g : integer = UInt(Pg);
///   let d : integer = UInt(Zd);
///   let merging : boolean = TRUE;
///   var imm : integer = SInt(imm8);
///   if sh == '1' then imm = imm << 8; end;
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   let dest : bits(VL) = Z[d];
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement{mask, e, esize} then
///           result[e*:esize] = imm[esize-1:0];
///       else
///           result[e*:esize] = dest[e*:esize];
///       end;
///   end;
///   Z[VL](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    primitives::PredicationMode,
    validate_instruction,
};

fn create_cpy_imm_instruction(mnemonic: &str, merging: bool) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let g = 0u8; // Pg
    let d = 0u8; // Zd

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let dest = Expr::ReadScalableVecReg { reg: d, vl };

    // Immediate: imm8 = 42, sh = 0 (no shift)
    let imm8 = 42i8;
    let imm = Expr::Const { value: imm8 as u128, ty: BitWidth::W64 }; // Sign-extended to 64-bit

    // First, create a vector with the immediate broadcast to all elements
    let broadcast_imm = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id, // Identity to copy broadcast value with predication
        predicate: Box::new(predicate),
        operand: Box::new(broadcast_imm),
        destination: Box::new(dest),
        vl,
        esize,
        predication: if merging {
            PredicationMode::Merging
        } else {
            PredicationMode::Zeroing
        },
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Copy signed integer immediate to vector elements ({})", if merging { "merging" } else { "zeroing" })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if merging { "cpy_z_p_i.md" } else { "cpy_z_o_i.md" }.to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// CPY (immediate, merging)
fn cpy_imm_merging_sve_instruction() -> Instruction {
    create_cpy_imm_instruction("CPY_IMM_MERGING", true)
}

// CPY (immediate, zeroing)
fn cpy_imm_zeroing_sve_instruction() -> Instruction {
    create_cpy_imm_instruction("CPY_IMM_ZEROING", false)
}

#[test]
fn cpy_imm_merging_sve_validates() {
    let instruction = cpy_imm_merging_sve_instruction();
    validate_instruction(&instruction).expect("CPY (immediate, merging) SVE should validate");
}

#[test]
fn cpy_imm_merging_sve_uses_merging() {
    let instruction = cpy_imm_merging_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { predication, .. } = value else {
        panic!("expected ScalableVectorMap for CPY immediate merging");
    };

    assert_eq!(*predication, PredicationMode::Merging);
}

#[test]
fn cpy_imm_zeroing_sve_validates() {
    let instruction = cpy_imm_zeroing_sve_instruction();
    validate_instruction(&instruction).expect("CPY (immediate, zeroing) SVE should validate");
}

#[test]
fn cpy_imm_zeroing_sve_uses_zeroing() {
    let instruction = cpy_imm_zeroing_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { predication, .. } = value else {
        panic!("expected ScalableVectorMap for CPY immediate zeroing");
    };

    assert_eq!(*predication, PredicationMode::Zeroing);
}

// CPY (scalar) - Copy general-purpose register to vector elements (predicated)
/// Stanford source: arm64_md_clean/cpy_z_p_r.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);
///   let g : integer = UInt(Pg);
///   let n : integer = UInt(Rn);
///   let d : integer = UInt(Zd);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   var result : bits(VL) = Z[d];
///   if AnyActiveElement(mask, esize) then
///       var operand1 : bits(64);
///       if n == 31 then operand1 = SP[64]() else operand1 = X[n]; end;
///       for e = 0 to elements-1 do
///           if ActivePredicateElement{mask, e, esize} then
///               result[e*:esize] = operand1[esize-1:0];
///           end;
///       end;
///   end;
///   Z[VL](d) = result.

fn cpy_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let g = 0u8; // Pg
    let n = 1u8; // Rn (not SP)
    let d = 0u8; // Zd

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let dest = Expr::ReadScalableVecReg { reg: d, vl };

    // Source register X1 (not SP)
    let operand1 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 };

    // First, broadcast the scalar register value to a full vector
    let broadcast_operand1 = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(operand1)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id, // Identity to copy broadcast value with predication
        predicate: Box::new(predicate),
        operand: Box::new(broadcast_operand1),
        destination: Box::new(Expr::ReadScalableVecReg { reg: d, vl }),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CPY_SCALAR".to_string(),
            encoding: Some("Copy general-purpose register to vector elements (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cpy_z_p_r.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cpy_scalar_sve_validates() {
    let instruction = cpy_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CPY (scalar) SVE should validate");
}

#[test]
fn cpy_scalar_sve_uses_merging() {
    let instruction = cpy_scalar_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { predication, .. } = value else {
        panic!("expected ScalableVectorMap for CPY scalar");
    };

    assert_eq!(*predication, PredicationMode::Merging);
}

#[test]
fn cpy_scalar_sve_uses_gpr_source() {
    let instruction = cpy_scalar_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { operand, .. } = value else {
        panic!("expected ScalableVectorMap for CPY scalar");
    };

    // The operand should be the broadcast result (SVESpecial::Dup)
    let Expr::SVESpecial { op, operands, .. } = operand.as_ref() else {
        panic!("expected SVESpecial for broadcast");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert!(matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(1), width: BitWidth::W64 }));
}

// CPY (SIMD&FP scalar) - Copy SIMD&FP scalar register to vector elements (predicated)
/// Stanford source: arm64_md_clean/cpy_z_p_v.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);
///   let g : integer = UInt(Pg);
///   let n : integer = UInt(Vn);
///   let d : integer = UInt(Zd);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   let operand1 : bits(esize) = if AnyActiveElement(mask, esize) then V[esize](n) else Zeros{esize};
///   var result : bits(VL) = Z[d];
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(mask, e, esize) then
///           result[e*:esize] = operand1;
///       end;
///   end;
///   Z[VL](d) = result.

fn cpy_simd_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let g = 0u8; // Pg
    let n = 1u8; // Vn
    let d = 0u8; // Zd

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let dest = Expr::ReadScalableVecReg { reg: d, vl };

    // SIMD&FP scalar register V1
    let operand1 = Expr::ReadReg { reg: semantic_ir::types::Reg::V(1), width: esize };

    // First, broadcast the SIMD scalar register value to a full vector
    let broadcast_operand1 = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(operand1)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let result = Expr::ScalableVectorMap {
        op: semantic_ir::primitives::SVEVectorUnaryOp::Id, // Identity to copy broadcast value with predication
        predicate: Box::new(predicate),
        operand: Box::new(broadcast_operand1),
        destination: Box::new(Expr::ReadScalableVecReg { reg: d, vl }),
        vl,
        esize,
        predication: PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CPY_SIMD_SCALAR".to_string(),
            encoding: Some("Copy SIMD&FP scalar register to vector elements (predicated, merging)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cpy_z_p_v.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cpy_simd_scalar_sve_validates() {
    let instruction = cpy_simd_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CPY (SIMD&FP scalar) SVE should validate");
}

#[test]
fn cpy_simd_scalar_sve_uses_v_reg() {
    let instruction = cpy_simd_scalar_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap { operand, .. } = value else {
        panic!("expected ScalableVectorMap for CPY SIMD scalar");
    };

    // The operand should be the broadcast result (SVESpecial::Dup)
    let Expr::SVESpecial { op, operands, .. } = operand.as_ref() else {
        panic!("expected SVESpecial for broadcast");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert!(matches!(operands[0].as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::V(1), width: BitWidth::W32 }));
}