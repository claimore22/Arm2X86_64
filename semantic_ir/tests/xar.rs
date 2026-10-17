/// XAR: Vector extract and rotate
/// Stanford source: xar.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn xar_instruction() -> Instruction {
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
    // XAR: Vector extract and rotate
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPAdd, // Placeholder for XAR
        left: Box::new(left),
        right: Box::new(right),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };
    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "XAR".to_string(),
            encoding: Some("Vector extract and rotate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "xar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn xar_validates() {
    let instruction = xar_instruction();
    validate_instruction(&instruction).expect("XAR should validate");
}

#[test]
fn xar_uses_vector_map2() {
    let instruction = xar_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for XAR");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
}
