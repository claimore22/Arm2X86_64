/// USHLLB (unpredicated) - Unsigned shift left long by immediate (bottom)
/// Stanford source: arm64_md_clean/ushllb_z_zi.md
/// USHLLT (unpredicated) - Unsigned shift left long by immediate (top)
/// Stanford source: arm64_md_clean/ushllt_z_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ushllb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size
    let dbl_esize = BitWidth::W64; // Destination element size (2x)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // USHLLB: shift even-numbered elements left by immediate, place in double-width elements
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount (0 to esize-1)

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ushllb,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand),      // Zn - source
        middle: Box::new(shift_amount), // immediate shift amount
        right: Box::new(destination.clone()), // Zd - destination
        destination: Box::new(destination),
        vl,
        esize: dbl_esize, // Destination element size (2x)
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "USHLLB".to_string(),
            encoding: Some("Unsigned shift left long by immediate (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ushllb_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ushllt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size
    let dbl_esize = BitWidth::W64; // Destination element size (2x)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // USHLLT: shift odd-numbered elements left by immediate, place in double-width elements
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount (0 to esize-1)

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Ushllt,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // unpredicated
        left: Box::new(operand),      // Zn - source
        middle: Box::new(shift_amount), // immediate shift amount
        right: Box::new(destination.clone()), // Zd - destination
        destination: Box::new(destination),
        vl,
        esize: dbl_esize, // Destination element size (2x)
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "USHLLT".to_string(),
            encoding: Some("Unsigned shift left long by immediate (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ushllt_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ushllb_sve_validates() {
    let instruction = ushllb_sve_instruction();
    validate_instruction(&instruction).expect("USHLLB SVE should validate");
}

#[test]
fn ushllt_sve_validates() {
    let instruction = ushllt_sve_instruction();
    validate_instruction(&instruction).expect("USHLLT SVE should validate");
}

#[test]
fn ushllb_sve_structure() {
    let instruction = ushllb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for USHLLB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Ushllb);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::Const { value: 4, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}

#[test]
fn ushllt_sve_structure() {
    let instruction = ushllt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for USHLLT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Ushllt);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::Const { value: 4, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}