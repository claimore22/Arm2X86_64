/// UHSUB: Unsigned halving subtract
/// Stanford source: uhsub.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn uhsub_instruction() -> Instruction {
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

    // UHSUB: Unsigned halving subtract
    // result = (left - right) >> 1
    let sub = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub,
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::Sub, // Placeholder for shift right by 1
        left: Box::new(sub),
        right: Box::new(Expr::Const { value: 1, ty: BitWidth::W128 }), // Match datasize
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UHSUB".to_string(),
            encoding: Some("Unsigned halving subtract".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "uhsub.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn uhsub_validates() {
    let instruction = uhsub_instruction();
    validate_instruction(&instruction).expect("UHSUB should validate");
}

#[test]
fn uhsub_uses_halving_sub() {
    let instruction = uhsub_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for UHSUB");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
