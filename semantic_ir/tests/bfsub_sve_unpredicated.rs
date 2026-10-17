/// BFSUB (unpredicated) - BFloat16 subtract (unpredicated)
/// Stanford source: arm64_md_clean/bfsub_z_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV 16;
///   let operand1 : bits(VL) = Z(n);
///   let operand2 : bits(VL) = Z(m);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(16) = operand1[e*16];
///       let element2 : bits(16) = operand2[e*16];
///       result[e*16] = BFSub(16)(element1, element2, FPCR());
///   end;
///   Z(VL)(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bfsub_sve_unpredicated_instruction() -> Instruction {
    // Using VL=256, esize=16 (16 elements for BFloat16)
    let vl = BitWidth::W256;
    let esize = BitWidth::W16;

    // Read first source vector Z1
    let operand1 = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };

    // Read second source vector Z2
    let operand2 = Expr::ReadScalableVecReg {
        reg: 2,
        vl,
    };

    // Read destination vector Z0
    let destination = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // ScalableVectorMap2 with SVEVectorBinOp::Bfloat16Sub (unpredicated - no predicate)
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Bfloat16Sub,
        predicate: Box::new(Expr::Const { value: 0, ty: BitWidth::W32 }), // No predicate - all active
        left: Box::new(operand1),
        right: Box::new(operand2),
        destination: Box::new(destination),
        vl,
        esize,
        predication: semantic_ir::primitives::PredicationMode::Zeroing, // Unpredicated = zeroing
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFSUB".to_string(),
            encoding: Some("BFloat16 subtract (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfsub_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfsub_sve_unpredicated_validates() {
    let instruction = bfsub_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BFSUB SVE unpredicated should validate");
}

#[test]
fn bfsub_sve_unpredicated_uses_scalable_vector_map2() {
    let instruction = bfsub_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { op, predicate: _, left, right, destination, vl, esize, predication } = value else {
        panic!("expected ScalableVectorMap2 for BFSUB SVE unpredicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorBinOp::Bfloat16Sub);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W16);
    assert_eq!(*predication, semantic_ir::primitives::PredicationMode::Zeroing);
    assert!(matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn bfsub_sve_unpredicated_element_count() {
    let instruction = bfsub_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    // VL=256, esize=16 -> 16 elements
    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 16);
}