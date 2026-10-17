/// ADCLB - Add with carry long (bottom)
/// Stanford source: arm64_md_clean/adclb_z_zzz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn adclb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // Element size (32-bit)

    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let carries = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm (carry input)
    let addend = Expr::ReadScalableVecReg { reg: 0, vl }; // Zda (destination/accumulator)

    // ADCLB: for each pair of elements (bottom), element + operand + carry_in
    // Produces result in even elements, carry_out in odd elements
    // Using SVEVectorTernaryOp::Adclb as the operation
    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Adclb,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // Unpredicated
        left: Box::new(operand),
        middle: Box::new(carries),
        right: Box::new(addend.clone()),
        destination: Box::new(addend),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADCLB".to_string(),
            encoding: Some("Add with carry long (bottom)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adclb_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adclb_sve_validates() {
    let instruction = adclb_sve_instruction();
    validate_instruction(&instruction).expect("ADCLB SVE should validate");
}

#[test]
fn adclb_sve_structure() {
    let instruction = adclb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::ScalableVectorMap3 { op, predicate, left, middle, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap3 for ADCLB");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Adclb);
    matches!(&**predicate, Expr::Const { value: 0, ty: BitWidth::W32 });
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(middle.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}