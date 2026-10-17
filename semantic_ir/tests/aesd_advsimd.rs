/// AESD - AES single round decryption
/// Stanford source: arm64_md_clean/aesd_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(128) = V{}(d);
///   let operand2 : bits(128) = V{}(n);
///   var result : bits(128) = operand1 XOR operand2;
///   result = AESInvShiftRows(result);
///   V{128}(d) = AESInvSubBytes(result);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn aesd_advsimd_instruction() -> Instruction {
    let operand1 = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // Step 1: XOR
    let xor_result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,  // 128-bit elements
    };

    // Step 2: AESInvShiftRows
    let shift_rows_result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::AESInvShiftRows,
        operands: vec![xor_result],
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
        imm: None,
    };

    // Step 3: AESInvSubBytes
    let final_result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::AESInvSubBytes,
        operands: vec![shift_rows_result],
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: final_result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AESD".to_string(),
            encoding: Some("AES single round decryption".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesd_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesd_advsimd_validates() {
    let instruction = aesd_advsimd_instruction();
    validate_instruction(&instruction).expect("AESD should validate");
}

#[test]
fn aesd_advsimd_uses_xor_shiftrows_subbytes() {
    let instruction = aesd_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Final result is AESInvSubBytes
    let Expr::VectorPerm { op, operands, datasize, esize, .. } = value else {
        panic!("expected VectorPerm for AESInvSubBytes");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::AESInvSubBytes);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert_eq!(operands.len(), 1);

    // Input to AESInvSubBytes is AESInvShiftRows
    let Expr::VectorPerm { op: op2, operands: operands2, .. } = &operands[0] else {
        panic!("expected VectorPerm for AESInvShiftRows");
    };

    assert_eq!(*op2, semantic_ir::primitives::VectorPermOp::AESInvShiftRows);
    assert_eq!(operands2.len(), 1);

    // Input to AESInvShiftRows is XOR
    let Expr::VectorMap2 { op: op3, left, right, datasize, esize } = &operands2[0] else {
        panic!("expected VectorMap2 for XOR");
    };

    assert_eq!(*op3, semantic_ir::primitives::VectorBinOp::Xor);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}