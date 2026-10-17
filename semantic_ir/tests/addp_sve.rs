/// ADDP (SVE predicated) - Add pairwise
/// Stanford source: arm64_md_clean/addp_z_p_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   let operand1 : bits(VL) = Z[dn];
///   let operand2 : bits(VL) = if AnyActiveElement(mask, esize) then Z[m] else Zeros{VL};
///   var result : bits(VL);
///   var element1 : integer; var element2 : integer;
///   for e = 0 to elements-1 do
///       if !ActivePredicateElement(mask, e, esize) then
///           result[e*:esize] = operand1[e*:esize];
///       else if IsEven(e) then
///           element1 = UInt(operand1[(e + 0)*:esize]);
///           element2 = UInt(operand1[(e + 1)*:esize]);
///       else
///           element1 = UInt(operand2[(e - 1)*:esize]);
///           element2 = UInt(operand2[(e + 0)*:esize]);
///       end;
///       let res : integer = element1 + element2;
///       result[e*:esize] = res[esize-1:0];
///   end;
///   Z[VL](dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn addp_sve_predicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32; // VL/8
    let esize = BitWidth::W64; // 64-bit elements
    let elements = vl.bits() / esize.bits(); // 4 elements

    let g = 0u8; // Pg
    let m = 1u8; // Zm
    let dn = 0u8; // Zdn

    // Predicate register
    let predicate = Expr::ReadPredReg { reg: g, pl };

    // Source vectors
    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: m, vl };

    // Destination (same as operand1 for destructive operation)
    let destination = Expr::ReadScalableVecReg { reg: dn, vl };

    // Use ScalableVectorPairwise for pairwise addition
    // For even e: adds operand1[e] + operand1[e+1]
    // For odd e: adds operand2[e-1] + operand2[e]
    // Results interleaved
    let result = Expr::ScalableVectorPairwise {
        op: semantic_ir::primitives::SVEVectorPairwiseOp::AddP,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Merging,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: dn,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDP".to_string(),
            encoding: Some("Add pairwise (SVE predicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addp_z_p_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addp_sve_predicated_validates() {
    let instruction = addp_sve_predicated_instruction();
    validate_instruction(&instruction).expect("ADDP SVE predicated should validate");
}

#[test]
fn addp_sve_predicated_uses_scalable_vector_pairwise() {
    let instruction = addp_sve_predicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorPairwise { op, vl, esize, predication, .. } = value else {
        panic!("expected ScalableVectorPairwise for ADDP SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorPairwiseOp::AddP);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Merging);
}

// ADDPL - Add multiple of predicate register size to scalar register
/// Stanford source: arm64_md_clean/addpl_r_ri.md
///
/// Stanford pseudocode:
///   let n : integer = UInt(Rn);
///   let d : integer = UInt(Rd);
///   let imm : integer = SInt(imm6);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let operand1 : bits(64) = if n == 31 then SP[64]() else X[64](n);
///   let result : bits(64) = operand1 + (imm * (PL DIV 8));
///   if d == 31 then SP[64]() = result; else X[64](d) = result; end;

fn addpl_scalar_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32; // VL/8
    // PL DIV 8 = 4 (in bytes)

    let n = 0u8; // Rn (not SP)
    let d = 1u8; // Rd (not SP)
    let imm = 5i64; // imm6 = 5 (positive example)

    let operand1 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(n), width: BitWidth::W64 };

    // PL/8 = 4 bytes
    let pl_bytes = Expr::Const { value: (pl.bits() / 8) as u128, ty: BitWidth::W64 };
    let imm_expr = Expr::Const { value: imm as u128, ty: BitWidth::W64 };

    // result = operand1 + (imm * (PL/8))
    let pl_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(pl_bytes),
        right: Box::new(imm_expr),
        ty: semantic_ir::types::ValueType::Bits(semantic_ir::types::Width::Fixed(BitWidth::W64)),
    };

    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(operand1),
        right: Box::new(pl_times_imm),
        ty: semantic_ir::types::ValueType::Bits(semantic_ir::types::Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![semantic_ir::stmt::Stmt::WriteReg {
        reg: semantic_ir::types::Reg::X(d),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDPL".to_string(),
            encoding: Some("Add multiple of predicate register size to scalar register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addpl_r_ri.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addpl_scalar_sve_validates() {
    let instruction = addpl_scalar_sve_instruction();
    validate_instruction(&instruction).expect("ADDPL SVE should validate");
}

#[test]
fn addpl_scalar_sve_uses_pl_bytes() {
    let instruction = addpl_scalar_sve_instruction();

    let semantic_ir::stmt::Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // result = operand1 + (imm * (PL/8))
    let semantic_ir::expr::Expr::Binary { op: semantic_ir::BinOp::Add, left, right, .. } = value else {
        panic!("expected Binary Add for result");
    };

    // Left should be the source register
    assert!(matches!(left.as_ref(), semantic_ir::expr::Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 }));

    // Right should be imm * (PL/8)
    let semantic_ir::expr::Expr::Binary { op: semantic_ir::BinOp::Mul, left: pl_bytes, right: imm_expr, .. } = right.as_ref() else {
        panic!("expected Binary Mul for PL/8 * imm");
    };

    // PL/8 should be 4 (256/8/8 = 4)
    assert!(matches!(pl_bytes.as_ref(), semantic_ir::expr::Expr::Const { value: 4, ty: BitWidth::W64 }));
    
    // imm should be 5
    assert!(matches!(imm_expr.as_ref(), semantic_ir::expr::Expr::Const { value, ty: BitWidth::W64, .. } if *value as i64 == 5));
}