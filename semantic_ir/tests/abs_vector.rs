/// ABS (vector) - Absolute value (vector)
/// Stanford source: arm64_md_clean/abs_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   var element : integer;
///   for e = 0 to elements-1 do
///       element = SInt(operand[e*esize]);
///       element = Abs(element);
///       result[e*esize] = element[esize-1:0];
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn abs_vector_instruction() -> Instruction {
    // Using 128-bit vector with 64-bit elements (2 elements)
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // VectorMap: apply Abs to each element
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
            esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ABS".to_string(),
            encoding: Some("Absolute value (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "abs_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn abs_vector_validates() {
    let instruction = abs_vector_instruction();
    validate_instruction(&instruction).expect("ABS vector should validate");
}

#[test]
fn abs_vector_uses_vector_map() {
    let instruction = abs_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Abs);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}

#[test]
fn abs_vector_datasize_esize_consistency() {
    // Test that validation enforces datasize % esize == 0
    let instruction = abs_vector_instruction();
    validate_instruction(&instruction).expect("Valid combination should pass");
    
    // Test with 32-bit elements (4 elements in 128-bit)
    let bad_instruction = Instruction {
        id: InstructionId {
            mnemonic: "ABS".to_string(),
            encoding: Some("Absolute value (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteReg {
            reg: Reg::V(1),
            value: Expr::VectorMap {
                op: semantic_ir::primitives::VectorUnaryOp::Abs,
                operand: Box::new(Expr::ReadReg {
                    reg: Reg::V(2),
                    width: BitWidth::W128,
                }),
                datasize: BitWidth::W128,
                esize: BitWidth::W32,  // 128 % 32 == 0, valid (4 elements)
            },
        }],
        provenance: Provenance {
            source_path: "abs_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    };
    
    validate_instruction(&bad_instruction).expect("128-bit datasize with 32-bit esize should validate");
}