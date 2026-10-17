/// VCVT (integer to floating-point): Integer convert to floating-point
/// Stanford source: vcvt_int_fp.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn vcvt_int_fp_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    // VCVT: Integer convert to floating-point
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Abs, // Placeholder for VCVT
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "VCVT (int to FP)".to_string(),
            encoding: Some("Integer convert to floating-point".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FP".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "vcvt_int_fp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn vcvt_int_fp_validates() {
    let instruction = vcvt_int_fp_instruction();
    validate_instruction(&instruction).expect("VCVT (int to FP) should validate");
}

#[test]
fn vcvt_int_fp_uses_vector_map() {
    let instruction = vcvt_int_fp_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for VCVT (int to FP)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}
