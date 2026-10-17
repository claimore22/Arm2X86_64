/// FRECPS: Floating-point reciprocal step
/// Stanford source: frecps.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frecps_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let left = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };
    let right = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // FRECPS: Reciprocal step - used with FRECPE to refine reciprocal
    // result = 2 - left * right
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    // Constant 2.0 broadcast to all 4 lanes (128-bit)
    // 2.0f32 = 0x40000000, broadcast to 4 lanes = 0x40000000400000004000000040000000
    let const_two = Expr::Const {
        value: 0x40000000_40000000_40000000_40000000u128,
        ty: BitWidth::W128,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPSub,
        left: Box::new(const_two),
        right: Box::new(mul),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRECPS".to_string(),
            encoding: Some("Floating-point reciprocal step".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frecps.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frecps_validates() {
    let instruction = frecps_instruction();
    validate_instruction(&instruction).expect("FRECPS should validate");
}

#[test]
fn frecps_uses_recip_step() {
    let instruction = frecps_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FRECPS");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPSub);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(left.as_ref(), Expr::Const { .. }));

    // Check right is FPMul
    let Expr::VectorMap2 { op: mul_op, left: mul_left, right: mul_right, .. } = right.as_ref() else {
        panic!("expected VectorMap2 for multiply");
    };
    assert_eq!(*mul_op, semantic_ir::primitives::VectorBinOp::FPMul);
}