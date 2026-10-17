/// BIC (vector, immediate) - Bitwise bit clear (vector, immediate)
/// Stanford source: arm64_md_clean/bic_advsimd_imm.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(rd);
///   [V]{datasize}(rd) = operand AND NOT(imm);
///   // imm = Replicate(AdvSIMDExpandImm(op, cmode, a::b::c::d::e::f::g::h))
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bic_advsimd_imm_instruction() -> Instruction {
    let datasize = BitWidth::W128;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // Immediate expansion (AdvSIMDExpandImm) - modeled as a constant
    // In practice, this would be expanded from the encoding fields
    let imm = Expr::Const {
        value: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128,  // Example 128-bit immediate
        ty: BitWidth::W128,
    };

    // NOT(imm)
    let not_imm = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(imm),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    // operand AND NOT(imm)
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::And,
        left: Box::new(operand),
        right: Box::new(not_imm),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(2),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BIC".to_string(),
            encoding: Some("Bitwise bit clear (vector, immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bic_advsimd_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bic_advsimd_imm_validates() {
    let instruction = bic_advsimd_imm_instruction();
    validate_instruction(&instruction).expect("BIC (vector, immediate) should validate");
}

#[test]
fn bic_advsimd_imm_uses_and_not() {
    let instruction = bic_advsimd_imm_instruction();
    
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

    // Right side is NOT(imm)
    let Expr::VectorMap { op: not_op, operand, .. } = right.as_ref() else {
        panic!("expected VectorMap for NOT");
    };

    assert_eq!(*not_op, semantic_ir::primitives::VectorUnaryOp::Not);
    assert!(matches!(operand.as_ref(), Expr::Const { ty: BitWidth::W128, .. }));
}