/// AESIMC - AES inverse mix columns
/// Stanford source: arm64_md_clean/aesimc_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(128) = V{}(n);
///   V{128}(d) = AESInvMixColumns(operand);
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn aesimc_advsimd_instruction() -> Instruction {
    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::AESInvMixColumns,
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
            mnemonic: "AESIMC".to_string(),
            encoding: Some("AES inverse mix columns".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesimc_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesimc_advsimd_validates() {
    let instruction = aesimc_advsimd_instruction();
    validate_instruction(&instruction).expect("AESIMC should validate");
}

#[test]
fn aesimc_advsimd_uses_inv_mix_columns() {
    let instruction = aesimc_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPerm { op, operands, datasize, esize, .. } = value else {
        panic!("expected VectorPerm for AESInvMixColumns");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::AESInvMixColumns);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert_eq!(operands.len(), 1);
    assert!(matches!(&operands[0], Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}