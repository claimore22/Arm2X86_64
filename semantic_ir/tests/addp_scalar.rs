/// ADDP (scalar) - Add pair of elements (scalar)
/// Stanford source: arm64_md_clean/addp_advsimd_pair.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);  // datasize=128
///   [V]{esize}(d) = IntReduce{esize, datasize}(ReduceOp_ADD, operand);
///   // IntReduce: adds adjacent elements, reduces 128-bit (2x64) to 64-bit scalar
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn addp_scalar_instruction() -> Instruction {
    // ADDP scalar: 128-bit vector with 2x64-bit elements, reduces to 64-bit scalar
    let datasize = BitWidth::W128;  // Source vector size
    let esize = BitWidth::W64;      // Element size

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // VectorPairwise: reduce adjacent elements (2x64 -> 1x64)
    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::AddP,
        operand: Box::new(operand),
        operand_datasize: BitWidth::W128,
        result_datasize: BitWidth::W64,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDP".to_string(),
            encoding: Some("Add pair of elements (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addp_advsimd_pair.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addp_scalar_validates() {
    let instruction = addp_scalar_instruction();
    validate_instruction(&instruction).expect("ADDP scalar should validate");
}

#[test]
fn addp_scalar_uses_vector_pairwise() {
    let instruction = addp_scalar_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand, operand_datasize, result_datasize, esize } = value else {
        panic!("expected VectorPairwise");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::AddP);
    assert_eq!(*operand_datasize, BitWidth::W128);
    assert_eq!(*result_datasize, BitWidth::W64);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    // Note: result is written to X register (scalar 64-bit result)
}

#[test]
fn addp_scalar_reduces_two_elements() {
    // ADDP scalar takes 2 elements (128-bit / 64-bit = 2 elements) and reduces to 1
    let instruction = addp_scalar_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { operand_datasize, result_datasize, esize, .. } = value else {
        panic!("expected VectorPairwise");
    };

    // 128-bit datasize / 64-bit esize = 2 elements -> pairwise produces 1 element
    let elements = operand_datasize.bits() / esize.bits();
    assert_eq!(elements, 2);
}