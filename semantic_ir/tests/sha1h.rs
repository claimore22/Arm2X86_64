/// SHA1H: SHA1 fixed rotate
/// Stanford source: sha1h.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn sha1h_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: BitWidth::W128,
    };

    // SHA1H: SHA1 fixed rotate
    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Rev32, // Placeholder for SHA1H
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
            mnemonic: "SHA1H".to_string(),
            encoding: Some("SHA1 fixed rotate".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SHA1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sha1h.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sha1h_validates() {
    let instruction = sha1h_instruction();
    validate_instruction(&instruction).expect("SHA1H should validate");
}

#[test]
fn sha1h_uses_vector_map() {
    let instruction = sha1h_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for SHA1H");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(1), width: BitWidth::W128 }));
}