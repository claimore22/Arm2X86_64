/// ADR - Calculate vector address
/// Stanford source: arm64_md_clean/adr_z_az.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let base : bits(VL) = Z(n);
///   let offs : bits(VL) = Z(m);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let addr : bits(esize) = base[e*:esize];
///       let offselt : bits(osize) = offs[e*:esize][osize-1:0];
///       let offset : integer = if unsigned then UInt(offselt) else SInt(offselt);
///       result[e*:esize] = addr + (offset * mbytes);
///   end;
///   Z(VL)(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn adr_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64;

    let predicate = Expr::Const {
        value: 0,
        ty: BitWidth::W32,
    };

    let base = Expr::ReadScalableVecReg { reg: 0, vl };
    let offsets = Expr::ReadScalableVecReg { reg: 1, vl };
    let destination = Expr::ReadScalableVecReg { reg: 2, vl };

    // The IR models this as ScalableVectorMap2 with Add.
    // The offset extraction (low 32 bits), scaling (mbytes), and signed/unsigned
    // handling are lowering details not expressed in the IR.
    let result = Expr::ScalableVectorMap2 {
        op: semantic_ir::primitives::SVEVectorBinOp::Add,
        predicate: Box::new(predicate),
        left: Box::new(base),
        right: Box::new(offsets),
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
            mnemonic: "ADR".to_string(),
            encoding: Some("Calculate vector address".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adr_z_az.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adr_sve_validates() {
    let instruction = adr_sve_instruction();
    validate_instruction(&instruction).expect("ADR SVE should validate");
}

#[test]
fn adr_sve_uses_scalable_vector_map2() {
    let instruction = adr_sve_instruction();

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
        panic!("expected ScalableVectorMap2 for ADR SVE");
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
}

#[test]
fn adr_sve_element_count() {
    let instruction = adr_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap2 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap2");
    };

    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 4);
}
