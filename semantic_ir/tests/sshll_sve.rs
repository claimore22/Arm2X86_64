/// SSHLLB (unpredicated) - Signed shift left long by immediate (bottom)
/// Stanford source: arm64_md_clean/sshllb_z_zi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn sshllb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size
    let dbl_esize = BitWidth::W64; // Destination element size (2x)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // SSHLLB: shift even-numbered elements left by immediate, place in double-width elements
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount (0 to esize-1)

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sshllb,
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
            mnemonic: "SSHLLB".to_string(),
            encoding: Some("Signed shift left long by immediate (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sshllb_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn sshllt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Source element size
    let dbl_esize = BitWidth::W64; // Destination element size (2x)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn - source
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd - destination

    // SSHLLT: shift odd-numbered elements left by immediate, place in double-width elements
    let shift_amount = Expr::Const { value: 4, ty: BitWidth::W4 }; // shift amount (0 to esize-1)

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Sshllt,
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
            mnemonic: "SSHLLT".to_string(),
            encoding: Some("Signed shift left long by immediate (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "sshllt_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn sshllb_sve_validates() {
    let instruction = sshllb_sve_instruction();
    validate_instruction(&instruction).expect("SSHLLB SVE should validate");
}

#[test]
fn sshllt_sve_validates() {
    let instruction = sshllt_sve_instruction();
    validate_instruction(&instruction).expect("SSHLLT SVE should validate");
}

#[test]
fn sshllb_sve_structure() {
    let instruction = sshllb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for SSHLLB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Sshllb);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::Const { value: 4, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}

#[test]
fn sshllt_sve_structure() {
    let instruction = sshllt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, left, middle, right, destination, vl, esize, predication, predicate: _ } = value else {
        panic!("expected ScalableVectorMap3 for SSHLLT");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Sshllt);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::Const { value: 4, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}