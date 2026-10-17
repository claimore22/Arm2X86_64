/// AESE (vectors) - AES single round encryption
/// Stanford source: arm64_md_clean/aese_z_zz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let segments : integer = VL DIV 128;
///   let operand1 : bits(VL) = Z(dn);
///   let operand2 : bits(VL) = Z(m);
///   var result : bits(VL);
///   result = operand1 XOR operand2;
///   for s = 0 to segments-1 do
///       result[s*:128] = AESSubBytes(AESShiftRows(result[s*:128]));
///   end;
///   Z(VL)(dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn aese_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::AESSubBytes,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(BitWidth::W128),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "AESE".to_string(),
            encoding: Some("AES single round encryption (vectors)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_AES".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aese_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aese_sve_unpredicated_validates() {
    let instruction = aese_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("AESE SVE unpredicated should validate");
}

#[test]
fn aese_sve_unpredicated_uses_svespecial() {
    let instruction = aese_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial {
        op,
        operands,
        vl,
        esize,
..
    } = value
    else {
        panic!("expected SVESpecial for AESE SVE unpredicated");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::AESSubBytes);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W128));
    assert_eq!(operands.len(), 2);
    assert!(matches!(
        operands[0].as_ref(),
        Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }
    ));
    assert!(matches!(
        operands[1].as_ref(),
        Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }
    ));
}
