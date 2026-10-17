/// AESMC - AES mix columns
/// Stanford source: arm64_md_clean/aesmc_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(128) = V{}(n);
///   V{128}(d) = AESMixColumns(operand);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn aesmc_advsimd_instruction() -> Instruction {
    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::AESMixColumns,
        operands: vec![operand],
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AESMC".to_string(),
            encoding: Some("AES mix columns".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesmc_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesmc_advsimd_validates() {
    let instruction = aesmc_advsimd_instruction();
    validate_instruction(&instruction).expect("AESMC should validate");
}

#[test]
fn aesmc_advsimd_uses_mix_columns() {
    let instruction = aesmc_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPerm { op, operands, datasize, esize, .. } = value else {
        panic!("expected VectorPerm for AESMixColumns");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::AESMixColumns);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert_eq!(operands.len(), 1);
    assert!(matches!(&operands[0], Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}