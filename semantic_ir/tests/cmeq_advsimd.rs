/// CMEQ (register) - Compare bitwise equal (vector)
/// Stanford source: arm64_md_clean/cmeq_advsimd_reg.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element1 = operand1[e*esize];
///       element2 = operand2[e*esize];
///       result[e*esize] = if element1 == element2 then Ones{esize} else Zeros{esize};
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn cmeq_advsimd_reg_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::CMEQ,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CMEQ".to_string(),
            encoding: Some("Compare bitwise equal (vector, register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmeq_advsimd_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// CMEQ (zero) - Compare bitwise equal to zero (vector)
/// Stanford source: arm64_md_clean/cmeq_advsimd_zero.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element = SInt(operand[e*esize]);
///       result[e*esize] = if element == 0 then Ones{esize} else Zeros{esize};
///   end;
///   V{datasize}(d) = result;
fn cmeq_advsimd_zero_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // Compare with zero using unary CMEQZero
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::CMEQZero,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CMEQ".to_string(),
            encoding: Some("Compare bitwise equal to zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmeq_advsimd_zero.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmeq_advsimd_reg_validates() {
    let instruction = cmeq_advsimd_reg_instruction();
    validate_instruction(&instruction).expect("CMEQ (register) should validate");
}

#[test]
fn cmeq_advsimd_zero_validates() {
    let instruction = cmeq_advsimd_zero_instruction();
    validate_instruction(&instruction).expect("CMEQ (zero) should validate");
}

#[test]
fn cmeq_advsimd_reg_uses_vector_map2_cmeq() {
    let instruction = cmeq_advsimd_reg_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for CMEQ");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::CMEQ);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}

#[test]
fn cmeq_advsimd_zero_uses_vector_map_cmeq_zero() {
    let instruction = cmeq_advsimd_zero_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for CMEQZero");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::CMEQZero);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}