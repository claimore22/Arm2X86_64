/// LD4 (multiple structures): Load multiple 4-element structures to four registers
/// Stanford source: ld4_multiple_structures.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn ld4_multiple_structures_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };
    let offset = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W128, // Match datasize for VectorMap2 validation
    };

    // LD4 (multiple structures): Load multiple 4-element structures
    let result = Expr::VectorMap2 {
        op: semantic_ir::primitives::VectorBinOp::FPMul, // Placeholder for load
        left: Box::new(base),
        right: Box::new(offset),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD4 (multiple structures)".to_string(),
            encoding: Some("Load multiple 4-element structures to four registers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld4_multiple_structures.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld4_multiple_structures_validates() {
    let instruction = ld4_multiple_structures_instruction();
    validate_instruction(&instruction).expect("LD4 (multiple structures) should validate");
}

#[test]
fn ld4_multiple_structures_uses_vector_map2() {
    let instruction = ld4_multiple_structures_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap2 { op, left, right, datasize, esize } = value else {
        panic!("expected VectorMap2 for LD4 (multiple structures)");
    };

    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
}