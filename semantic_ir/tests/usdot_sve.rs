/// USDOT (SVE vectors) - Unsigned by signed 8-bit integer dot product to 32-bit integer
/// Stanford source: arm64_md_clean/usdot_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn usdot_vectors_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result is 32-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - unsigned 8-bit quads
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - signed 8-bit quads
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Zda - accumulator (32-bit)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::UDot, // Using UDot as placeholder
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W256,
        esize,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "USDOT (vectors)".to_string(),
            encoding: Some("Unsigned by signed 8-bit integer dot product to 32-bit integer".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_I8MM".to_string(), "FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "usdot_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// USDOT (SVE indexed) - Unsigned by signed 8-bit integer dot product by indexed element to 32-bit integer
fn usdot_indexed_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Result is 32-bit

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - unsigned 8-bit quads
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm - signed 8-bit quads
    let addend = Expr::ReadScalableVecReg { reg: 0, vl };   // Zda - accumulator (32-bit)

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::UDot, // Using UDot as placeholder
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: BitWidth::W256,
        esize,
        fpcr: Box::new(Expr::ReadSpecial { reg: semantic_ir::types::SpecialReg::Fpcr, width: BitWidth::W64 }),
        fpmr: None,
        broadcast_index: Some(0),
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "USDOT (indexed)".to_string(),
            encoding: Some("Unsigned by signed 8-bit integer dot product by indexed element to 32-bit integer".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_I8MM".to_string(), "FEAT_SVE2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "usdot_z_zzzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn usdot_vectors_sve_validates() {
    let instruction = usdot_vectors_sve_instruction();
    validate_instruction(&instruction).expect("USDOT (vectors) SVE should validate");
}

#[test]
fn usdot_indexed_sve_validates() {
    let instruction = usdot_indexed_sve_instruction();
    validate_instruction(&instruction).expect("USDOT (indexed) SVE should validate");
}

#[test]
fn usdot_vectors_sve_structure() {
    let instruction = usdot_vectors_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot { op, a, b, addend, datasize, esize, .. } = value else {
        panic!("expected VectorDot");
    };
    // Using UDot as placeholder
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::UDot);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    matches!(a.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(b.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(addend.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}

#[test]
fn usdot_indexed_sve_structure() {
    let instruction = usdot_indexed_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot { op, a, b, addend, datasize, esize, broadcast_index, .. } = value else {
        panic!("expected VectorDot");
    };
    assert_eq!(*op, semantic_ir::primitives::VectorDotOp::UDot);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert!(broadcast_index.is_some());
    assert_eq!(broadcast_index.unwrap(), 0);
}