/// FRSQRTS: Floating-point reciprocal square root step
/// Stanford source: frsqrts.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn frsqrts_instruction() -> Instruction {
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

    // FRSQRTS: Reciprocal square root step
    // result = 1.5 - 0.5 * left * right^2
    let right_sq = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(right.clone()),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(left),
        right: Box::new(right_sq),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    // Constant 0.5 broadcast to all 4 lanes (128-bit)
    // 0.5f32 = 0x3f000000, broadcast to 4 lanes = 0x3f0000003f0000003f0000003f000000
    let const_half = Expr::Const {
        value: 0x3f000000_3f000000_3f000000_3f000000u128,
        ty: BitWidth::W128,
    };
    let half_mul = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul,
        left: Box::new(const_half),
        right: Box::new(mul),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    // Constant 1.5 broadcast to all 4 lanes (128-bit)
    // 1.5f32 = 0x3fc00000, broadcast to 4 lanes = 0x3fc000003fc000003fc000003fc00000
    let const_one_half = Expr::Const {
        value: 0x3fc00000_3fc00000_3fc00000_3fc00000u128,
        ty: BitWidth::W128,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPSub,
        left: Box::new(const_one_half),
        right: Box::new(half_mul),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FRSQRTS".to_string(),
            encoding: Some("Floating-point reciprocal square root step".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "frsqrts.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn frsqrts_validates() {
    let instruction = frsqrts_instruction();
    validate_instruction(&instruction).expect("FRSQRTS should validate");
}

#[test]
fn frsqrts_uses_recip_sqrt_step() {
    let instruction = frsqrts_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for FRSQRTS");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorBinOp::FPSub);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}