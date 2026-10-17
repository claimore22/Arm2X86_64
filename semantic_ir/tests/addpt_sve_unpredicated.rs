/// ADDPT (unpredicated) - Add checked pointer vectors (unpredicated)
/// Stanford source: arm64_md_clean/addpt_z_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV 64;
///   let operand1 : bits(VL) = Z(n);
///   let operand2 : bits(VL) = Z(m);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(64) = operand1[e*:64];
///       let element2 : bits(64) = operand2[e*:64];
///       let res : bits(64) = element1 + element2;
///       result[e*:64] = PointerAddCheck(res, element1);
///   end;
///   Z(VL)(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn addpt_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let predicate = Expr::Const {
        value: 0,
        ty: BitWidth::W32,
    };

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 2,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDPT".to_string(),
            encoding: Some("Add checked pointer vectors (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE".to_string(),
            "FEAT_CPA".to_string(),
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addpt_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addpt_sve_unpredicated_validates() {
    let instruction = addpt_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("ADDPT SVE unpredicated should validate");
}

#[test]
fn addpt_sve_unpredicated_uses_zeroing_predication() {
    let instruction = addpt_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 {
        op,
        left,
        right,
        destination,
        vl,
        esize,
        predication,
..
    } = value
    else {
        panic!("expected ScalableVectorMap2 for ADDPT SVE unpredicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Add);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(
        *predication,
        semantic_ir::primitives::PredicationMode::Zeroing
    );
    assert!(matches!(
        left.as_ref(),
        Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        right.as_ref(),
        Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        destination.as_ref(),
        Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }
    ));
}

#[test]
fn addpt_sve_unpredicated_element_count() {
    let instruction = addpt_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 4);
}
