/// ADDP (vector) - Add pairwise (vector)
/// Stanford source: arm64_md_clean/addp_advsimd_vec.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   let concat : bits(2*datasize) = operand2 :: operand1;
///   for e = 0 to elements-1 do
///       element1 = concat[(2*e)*esize];
///       element2 = concat[((2*e)+1)*esize];
///       result[e*esize] = element1 + element2;
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn addp_vector_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let concat = Expr::Concat {
        high: Box::new(operand2),
        low: Box::new(operand1),
        width: BitWidth::W256,
    };

    let result = Expr::VectorPairwise {
        op: semantic_ir::primitives::VectorPairwiseOp::AddP,
        operand: Box::new(concat),
        operand_datasize: BitWidth::W256,
        result_datasize: BitWidth::W128,
        esize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDP".to_string(),
            encoding: Some("Add pairwise (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addp_advsimd_vec.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addp_vector_validates() {
    let instruction = addp_vector_instruction();
    validate_instruction(&instruction).expect("ADDP vector should validate");
}

#[test]
fn addp_vector_uses_concat_and_pairwise() {
    let instruction = addp_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { op, operand, operand_datasize, result_datasize, esize } = value else {
        panic!("expected VectorPairwise");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPairwiseOp::AddP);
    assert_eq!(*operand_datasize, BitWidth::W256);
    assert_eq!(*result_datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    
    let Expr::Concat { high, low, width } = operand.as_ref() else {
        panic!("expected Concat");
    };
    
    assert_eq!(*width, BitWidth::W256);
    assert!(matches!(high.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
    assert!(matches!(low.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}

#[test]
fn addp_vector_concat_order() {
    let instruction = addp_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPairwise { operand, .. } = value else {
        panic!("expected VectorPairwise");
    };

    let Expr::Concat { high, low, .. } = operand.as_ref() else {
        panic!("expected Concat");
    };

    // High = operand2 (V3), Low = operand1 (V2) per Stanford: concat = operand2 :: operand1
    assert!(matches!(high.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
    assert!(matches!(low.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}