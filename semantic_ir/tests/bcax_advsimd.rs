/// BCAX - Bit clear and exclusive-OR
/// Stanford source: arm64_md_clean/bcax_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(128) = V{}(m);
///   let operand2 : bits(128) = V{}(n);
///   let operand3 : bits(128) = V{}(a);
///   V{128}(d) = operand2 XOR (operand1 AND NOT(operand3));
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bcax_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let operand3 = Expr::ReadReg {
        reg: Reg::V(4),
        width: BitWidth::W128,
    };

    // operand1 AND NOT(operand3)
    let not_operand3 = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(operand3),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let and_result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::And,
        left: Box::new(operand1),
        right: Box::new(not_operand3),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    // operand2 XOR (operand1 AND NOT(operand3))
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(operand2),
        right: Box::new(and_result),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BCAX".to_string(),
            encoding: Some("Bit clear and exclusive-OR".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA3".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bcax_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bcax_advsimd_validates() {
    let instruction = bcax_advsimd_instruction();
    validate_instruction(&instruction).expect("BCAX should validate");
}

#[test]
fn bcax_advsimd_uses_and_not_xor() {
    let instruction = bcax_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Final result is XOR
    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for XOR");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Xor);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));

    // Right side is AND with NOT
    let Expr::VectorMap2 { op: and_op, left: and_left, right: and_right, .. } = right.as_ref() else {
        panic!("expected VectorMap2 for AND");
    };

    assert_eq!(*and_op, semantic_ir::primitives::VectorBinOp::And);
    assert!(matches!(and_left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));

    // AND right side is NOT
    let Expr::VectorMap { op: not_op, operand, .. } = and_right.as_ref() else {
        panic!("expected VectorMap for NOT");
    };

    assert_eq!(*not_op, semantic_ir::primitives::VectorUnaryOp::Not);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 }));
}