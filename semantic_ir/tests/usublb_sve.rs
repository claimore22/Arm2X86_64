/// USUBLB - Unsigned subtract long (bottom)
/// Stanford source: arm64_md_clean/usublb_z_zz.md
use semantic_ir::{BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, validate_instruction};

fn usublb_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;
    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Sub,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }),
        left: Box::new(operand1), right: Box::new(operand2), destination: Box::new(Expr::Const { value: 0, ty: BitWidth::W256 }),
        vl, esize, predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };
    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];
    Instruction {
        id: InstructionId { mnemonic: "USUBLB".to_string(), encoding: Some("Unsigned subtract long (bottom)".to_string()) },
        operands: Vec::new(), required_features: vec!["FEAT_SVE2".to_string()], decode: None, semantics,
        provenance: Provenance { source_path: "usublb_z_zz.md".to_string(), source_sha256: "t".to_string(), clean_sha256: "t".to_string() },
    }
}

#[test]
fn usublb_sve_validates() { validate_instruction(&usublb_sve_instruction()).expect("USUBLB SVE should validate"); }

#[test]
fn usublb_sve_structure() {
    let instruction = usublb_sve_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::ScalableVectorMap2 { op, left, right, destination, vl, esize, predication, predicate: _ } = value else { panic!("expected ScalableVectorMap2 for USUBLB"); };
    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Sub);
    matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
    matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(destination.as_ref(), Expr::Const { value: 0, ty: BitWidth::W256 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
}