/// CMGT (register) - Compare signed greater than (vector)
/// Stanford source: arm64_md_clean/cmgt_advsimd_reg.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element1 = SInt(operand1[e*esize]);
///       element2 = SInt(operand2[e*esize]);
///       result[e*esize] = if element1 > element2 then Ones{esize} else Zeros{esize};
///   end;
///   V{datasize}(d) = result.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn cmgt_advsimd_reg_instruction() -> Instruction {
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
        op: semantic_ir::primitives::VectorBinOp::CMGT,
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
            mnemonic: "CMGT".to_string(),
            encoding: Some("Compare signed greater than (vector, register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmgt_advsimd_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// CMGT (zero) - Compare signed greater than zero (vector)
/// Stanford source: arm64_md_clean/cmgt_advsimd_zero.md
fn cmgt_advsimd_zero_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::CMGTZero,
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
            mnemonic: "CMGT".to_string(),
            encoding: Some("Compare signed greater than zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmgt_advsimd_zero.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmgt_advsimd_reg_validates() {
    let instruction = cmgt_advsimd_reg_instruction();
    validate_instruction(&instruction).expect("CMGT (register) should validate");
}

#[test]
fn cmgt_advsimd_zero_validates() {
    let instruction = cmgt_advsimd_zero_instruction();
    validate_instruction(&instruction).expect("CMGT (zero) should validate");
}

#[test]
fn cmgt_advsimd_reg_uses_vector_map2_cmgt() {
    let instruction = cmgt_advsimd_reg_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for CMGT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::CMGT);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}

#[test]
fn cmgt_advsimd_zero_uses_vector_map_cmgt_zero() {
    let instruction = cmgt_advsimd_zero_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for CMGTZero");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::CMGTZero);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}