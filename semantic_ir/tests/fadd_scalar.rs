/// FADD (scalar) - Floating-point add (scalar)
/// Stanford source: arm64_md_clean/fadd_float.md
///
/// Stanford pseudocode:
///   let operand1 : bits(esize) = V{}(n);
///   let operand2 : bits(esize) = V{}(m);
///   var result : bits(128) = if IsMerging(FPCR()) then V{128}(n) else Zeros{128};
///   result[0+:esize] = FPAdd(esize)(operand1, operand2, FPCR());
///   V{128}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fadd_scalar_instruction() -> Instruction {
    // FADD scalar: single-precision (esize=32)
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

    // Extract the lower esize bits from each source register
    let elem1 = Expr::Extract {
        value: Box::new(operand1),
        lsb: 0,
        width: esize,
    };

    let elem2 = Expr::Extract {
        value: Box::new(operand2),
        lsb: 0,
        width: esize,
    };

    // Scalar FP add on the extracted elements
    let fp_add = Expr::ScalarFPBinary {
        op: semantic_ir::primitives::ScalarFPBinOp::FPAdd,
        left: Box::new(elem1),
        right: Box::new(elem2),
        ty: esize,
    };

    // Zero-extend the result to 128 bits (non-merging case)
    let result = Expr::ZeroExtend {
        value: Box::new(fp_add),
        to: datasize,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FADD".to_string(),
            encoding: Some("Floating-point add (scalar)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fadd_float.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fadd_scalar_validates() {
    let instruction = fadd_scalar_instruction();
    validate_instruction(&instruction).expect("FADD scalar should validate");
}

#[test]
fn fadd_scalar_uses_scalar_fp_binary() {
    let instruction = fadd_scalar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    // Result should be ZeroExtend of ScalarFPBinary
    let Expr::ZeroExtend { value: inner, to } = value else {
        panic!("expected ZeroExtend");
    };

    assert_eq!(*to, BitWidth::W128);

    let Expr::ScalarFPBinary { op, left, right, ty } = inner.as_ref() else {
        panic!("expected ScalarFPBinary");
    };

    assert_eq!(*op, semantic_ir::primitives::ScalarFPBinOp::FPAdd);
    assert_eq!(*ty, BitWidth::W32);

    // Left should be Extract from V2
    let Expr::Extract { value: left_val, lsb: left_lsb, width: left_width } = left.as_ref() else {
        panic!("expected Extract for left operand");
    };
    assert_eq!(*left_lsb, 0);
    assert_eq!(*left_width, BitWidth::W32);
    let Expr::ReadReg { reg: left_reg, width: left_reg_width } = left_val.as_ref() else {
        panic!("expected ReadReg for left operand");
    };
    assert_eq!(*left_reg, Reg::V(2));
    assert_eq!(*left_reg_width, BitWidth::W128);

    // Right should be Extract from V3
    let Expr::Extract { value: right_val, lsb: right_lsb, width: right_width } = right.as_ref() else {
        panic!("expected Extract for right operand");
    };
    assert_eq!(*right_lsb, 0);
    assert_eq!(*right_width, BitWidth::W32);
    let Expr::ReadReg { reg: right_reg, width: right_reg_width } = right_val.as_ref() else {
        panic!("expected ReadReg for right operand");
    };
    assert_eq!(*right_reg, Reg::V(3));
    assert_eq!(*right_reg_width, BitWidth::W128);
}