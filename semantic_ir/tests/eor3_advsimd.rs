/// EOR3 - Three-way exclusive-OR
/// Stanford source: arm64_md_clean/eor3_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(128) = V{}(m);
///   let operand2 : bits(128) = V{}(n);
///   let operand3 : bits(128) = V{}(a);
///   V{128}(d) = operand2 XOR operand1 XOR operand3;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn eor3_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W128;

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

    // EOR3: three-way XOR
    // First XOR operand1 and operand2, then XOR with operand3
    let xor_12 = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(xor_12),
        right: Box::new(operand3),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "EOR3".to_string(),
            encoding: Some("Three-way exclusive-OR".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA3".to_string(), "FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "eor3_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn eor3_advsimd_validates() {
    let instruction = eor3_advsimd_instruction();
    validate_instruction(&instruction).expect("EOR3 should validate");
}

#[test]
fn eor3_advsimd_uses_three_way_xor() {
    let instruction = eor3_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for final XOR");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Xor);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(4), width: BitWidth::W128 }));

    let Expr::VectorMap2 { op: xor_op, left: op1, right: op2, .. } = left.as_ref() else {
        panic!("expected VectorMap2 for first XOR");
    };

    assert_eq!(*xor_op, semantic_ir::primitives::VectorBinOp::Xor);
    assert!(matches!(op1.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(op2.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}