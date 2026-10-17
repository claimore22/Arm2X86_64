/// FADD (vector) - Floating-point add (vector)
/// Stanford source: arm64_md_clean/fadd_advsimd.md
///
/// Stanford pseudocode:
///   let operand1 : bits(datasize) = V{}(n);
///   let operand2 : bits(datasize) = V{}(m);
///   var result : bits(datasize);
///   var element1 : bits(esize);
///   var element2 : bits(esize);
///   for e = 0 to elements-1 do
///       element1 = operand1[e*esize];
///       element2 = operand2[e*esize];
///       result[e*esize] = FPAdd{esize}(element1, element2, FPCR());
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fadd_vector_instruction() -> Instruction {
    // FADD vector: single-precision (esize=32), 128-bit datasize (4 elements)
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // Single-precision

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // VectorMap2 with FPAdd: element-wise FP add
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FADD".to_string(),
            encoding: Some("Floating-point add (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadd_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fadd_vector_validates() {
    let instruction = fadd_vector_instruction();
    validate_instruction(&instruction).expect("FADD vector should validate");
}

#[test]
fn fadd_vector_uses_vector_map2_fpadd() {
    let instruction = fadd_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FADD vector");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPAdd);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}

#[test]
fn fadd_vector_elements() {
    // FADD vector with 128-bit datasize and 32-bit esize = 4 elements
    let instruction = fadd_vector_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { datasize, esize, .. } = value else {
        panic!("expected VectorMap2");
    };

    // 128-bit / 32-bit = 4 elements
    let elements = datasize.bits() / esize.bits();
    assert_eq!(elements, 4);
}