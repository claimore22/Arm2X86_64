/// ADCLT - Add with carry long (top)
/// Stanford source: arm64_md_clean/adclt_z_zzz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let pairs : integer = VL DIV (esize * 2);
///   let operand : bits(VL) = Z(n);
///   let carries : bits(VL) = Z(m);
///   var result : bits(VL) = Z(da);
///   for p = 0 to pairs-1 do
///       let element1 : bits(esize) = result[(2*p + 0)*esize];
///       let element2 : bits(esize) = operand[(2*p + 1)*esize];
///       let carry_in : bit = carries[(2*p + 1)*esize][0];
///       let (res, nzcv) = AddWithCarry{esize}(element1, element2, carry_in);
///       let carry_out : bit = nzcv[1];
///       result[(2*p + 0)*esize] = res;
///       result[(2*p + 1)*esize] = ZeroExtend{esize}(carry_out);
///   end;
///   Z(VL)(da) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn adclt_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32;

    let predicate = Expr::Const {
        value: 0,
        ty: BitWidth::W32,
    };

    let accumulator = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };
    let carries = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::ScalableVectorMap3 {
        op: semantic_ir::primitives::SVEVectorTernaryOp::Adclt,
        predicate: Box::new(predicate),
        left: Box::new(accumulator),
        middle: Box::new(operand),
        right: Box::new(carries),
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
            mnemonic: "ADCLT".to_string(),
            encoding: Some("Add with carry long (top)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "adclt_z_zzz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn adclt_sve_validates() {
    let instruction = adclt_sve_instruction();
    validate_instruction(&instruction).expect("ADCLT SVE should validate");
}

#[test]
fn adclt_sve_uses_scalable_vector_map3() {
    let instruction = adclt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap3 {
        op,
        left,
        middle,
        right,
        destination,
        vl,
        esize,
        predication,
..
    } = value
    else {
        panic!("expected ScalableVectorMap3 for ADCLT");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorTernaryOp::Adclt);
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
        middle.as_ref(),
        Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        right.as_ref(),
        Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        destination.as_ref(),
        Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }
    ));
}

#[test]
fn adclt_sve_pair_count() {
    let instruction = adclt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::ScalableVectorMap3 { vl, esize, .. } = value else {
        panic!("expected ScalableVectorMap3");
    };

    let pairs = vl.bits() / (esize.bits() * 2);
    assert_eq!(pairs, 4);
}
