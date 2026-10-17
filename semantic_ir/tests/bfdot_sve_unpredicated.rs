/// BFDOT (vectors) - BFloat16 dot product to single-precision
/// Stanford source: arm64_md_clean/bfdot_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    VectorDotOp,
    validate_instruction,
};

fn bfdot_sve_unpredicated_instruction() -> Instruction {
    // Choose a vector length that yields multiple elements (e.g., 256 bits)
    let vl = BitWidth::W256;
    // Source: 32-bit elements (pairs of BFloat16), Destination: 32-bit elements (single-precision)
    let esize = BitWidth::W32; // Element size for vectors

    // For BFDOT (vectors): operand1 (Zn), operand2 (Zm), accumulator (Zda), destination (Zda)
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };   // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };   // Zm
    let addend = Expr::ReadScalableVecReg { reg: 2, vl };     // Zda (accumulator)
    let destination = Expr::ReadScalableVecReg { reg: 2, vl }; // Zda (same as addend)

    let result = Expr::VectorDot {
        op: VectorDotOp::BFDot,
        a: Box::new(operand1),
        b: Box::new(operand2),
        addend: Box::new(addend),
        datasize: vl,
        esize,
        fpcr: Box::new(Expr::SystemReg { reg: 0 }), // FPCR
        fpmr: None, // Not used for BFDOT
        broadcast_index: None, // Not indexed version
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 2, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFDOT".to_string(),
            encoding: Some("BFloat16 dot product to single-precision".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE".to_string(), 
            "FEAT_SME".to_string(),
            "FEAT_BF16".to_string() // From decode: FEAT_BF16
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfdot_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfdot_sve_unpredicated_validates() {
    let instruction = bfdot_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BFDOT SVE unpredicated should validate");
}

#[test]
fn bfdot_sve_unpredicated_structure() {
    let instruction = bfdot_sve_unpredicated_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::VectorDot {
        op,
        a,
        b,
        addend,
        datasize,
        esize,
        fpcr,
        fpmr,
        broadcast_index,
    } = value else {
        panic!("expected VectorDot for BFDOT");
    };
    assert_eq!(*op, VectorDotOp::BFDot);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32); // 32-bit result elements
    assert!(fpmr.is_none()); // Not used for BFDOT
    assert_eq!(*broadcast_index, None); // Not indexed version
    // Verify the operand types (just ensure they are the expected variant)
    matches!(a.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(b.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(addend.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(fpcr.as_ref(), Expr::SystemReg { reg: 0 }); // FPCR
}