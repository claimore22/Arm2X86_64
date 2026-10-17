/// BIC (vector, register) - Bitwise bit clear (vector, register)
/// Stanford source: arm64_md_clean/bic_advsimd_reg.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = NOT(V{}(m));
///   V{datasize}(d) = operand1 AND operand2;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bic_advsimd_reg_instruction() -> Instruction {
    let datasize = BitWidth::W128;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // NOT(operand2)
    let not_operand2 = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    // operand1 AND NOT(operand2)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::And,
        left: Box::new(operand1),
        right: Box::new(not_operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BIC".to_string(),
            encoding: Some("Bitwise bit clear (vector, register)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bic_advsimd_reg.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bic_advsimd_reg_validates() {
    let instruction = bic_advsimd_reg_instruction();
    validate_instruction(&instruction).expect("BIC (vector, register) should validate");
}

#[test]
fn bic_advsimd_reg_uses_and_not() {
    let instruction = bic_advsimd_reg_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Final result is AND
    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for AND");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::And);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));

    // Right side is NOT(operand2)
    let Expr::VectorMap { op: not_op, operand, .. } = right.as_ref() else {
        panic!("expected VectorMap for NOT");
    };

    assert_eq!(*not_op, semantic_ir::primitives::VectorUnaryOp::Not);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}