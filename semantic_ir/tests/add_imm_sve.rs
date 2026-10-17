/// ADD (immediate) - Add immediate (unpredicated)
/// Stanford source: arm64_md_clean/add_z_zi.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let operand1 : bits(VL) = Z(dn);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(esize) = operand1[e*:esize];
///       result[e*:esize] = element1 + imm;
///   end;
///   Z(VL)(dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn add_imm_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::Const {
        value: 0,
        ty: BitWidth::W32,
    };

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Immediate value held in a vector register (broadcast to all elements)
    let imm = Expr::ReadScalableVecReg { reg: 2, vl };

    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(predicate),
        left: Box::new(operand1),
        right: Box::new(imm),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADD".to_string(),
            encoding: Some("Add immediate (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "add_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn add_imm_sve_validates() {
    let instruction = add_imm_sve_instruction();
    validate_instruction(&instruction).expect("ADD (immediate) SVE should validate");
}

#[test]
fn add_imm_sve_uses_scalable_vector_map2() {
    let instruction = add_imm_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 {
        op,
        left,
        right,
        vl,
        esize,
        predication,
..
    } = value
    else {
        panic!("expected ScalableVectorMap2 for ADD (immediate) SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Add);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
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
        Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }
    ));
}

#[test]
fn add_imm_sve_element_count() {
    let instruction = add_imm_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 8);
}
