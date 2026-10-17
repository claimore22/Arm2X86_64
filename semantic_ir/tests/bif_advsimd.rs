/// BIF - Bitwise insert if false
/// Stanford source: arm64_md_clean/bif_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(d);
///   let operand2 : bits(datasize) = NOT(V{}(m));
///   let operand3 : bits(datasize) = V{}(n);
///   V{datasize}(d) = operand1 XOR ((operand1 XOR operand3) AND operand2);
///   // If V(m) bit is 0: take from V(n); else keep V(d)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn bif_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;

    let operand1 = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    let operand3 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand_m = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let not_m = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(Expr::ReadReg {
            reg: Reg::V(3),
            width: BitWidth::W128,
        }),
        datasize,
        esize: datasize,
    };

    let xor_13 = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(Expr::ReadReg {
            reg: Reg::V(1),
            width: BitWidth::W128,
        }),
        right: Box::new(Expr::ReadReg {
            reg: Reg::V(2),
            width: BitWidth::W128,
        }),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let not_m = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Not,
        operand: Box::new(Expr::ReadReg {
            reg: Reg::V(3),
            width: BitWidth::W128,
        }),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let and_result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::And,
        left: Box::new(xor_13),
        right: Box::new(not_m),
        datasize: BitWidth::W128,
        esize: BitWidth::W128,
    };

    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Xor,
        left: Box::new(Expr::ReadReg {
            reg: Reg::V(1),
            width: BitWidth::W128,
        }),
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
            mnemonic: "BIF".to_string(),
            encoding: Some("Bitwise insert if false".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bif_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bif_advsimd_validates() {
    let instruction = bif_advsimd_instruction();
    validate_instruction(&instruction).expect("BIF should validate");
}

#[test]
fn bif_advsimd_uses_xor_and_not() {
    let instruction = bif_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for final XOR");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::Xor);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));

    let Expr::VectorMap2 { op: and_op, left: xor_13, right: not_m, .. } = right.as_ref() else {
        panic!("expected VectorMap2 for AND");
    };

    assert_eq!(*and_op, semantic_ir::primitives::VectorBinOp::And);

    let Expr::VectorMap2 { op: xor_op, left: v1, right: v2, .. } = xor_13.as_ref() else {
        panic!("expected VectorMap2 for XOR");
    };

    assert_eq!(*xor_op, semantic_ir::primitives::VectorBinOp::Xor);
    assert!(matches!(v1.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
    assert!(matches!(v2.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));

    let Expr::VectorMap { op: not_op, operand, .. } = not_m.as_ref() else {
        panic!("expected VectorMap for NOT");
    };

    assert_eq!(*not_op, semantic_ir::primitives::VectorUnaryOp::Not);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}