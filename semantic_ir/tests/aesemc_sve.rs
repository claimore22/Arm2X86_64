/// AESEMC - Multi-vector AES single round encryption and mix columns
/// Stanford source: arm64_md_clean/aesemc_mz_zzi.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let segments : integer = VL DIV 128;
///   let operand2 : bits(VL) = Z(m);
///   for r = 0 to nreg-1 do
///       let operand1 : bits(VL) = Z(dn + r);
///       for s = 0 to segments-1 do
///           let keyindex : integer = (s - (s MOD 4)) + index;
///           let res : bits(128) = operand1[s*:128] XOR operand2[keyindex*:128];
///           results[[r]][s*:128] = AESMixColumns(AESSubBytes(AESShiftRows(res)));
///       end;
///   end;
///   for r = 0 to nreg-1 do Z(VL)(dn + r) = results[[r]]; end;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn aesemc_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl };

    // The IR models this as SVESpecial with AESMixColumns.
    // The SubBytes and ShiftRows operations are applied in the lowering pass.
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::AESMixColumns,
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
            mnemonic: "AESEMC".to_string(),
            encoding: Some("Multi-vector AES single round encryption and mix columns".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_AES2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "aesemc_mz_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn aesemc_sve_validates() {
    let instruction = aesemc_sve_instruction();
    validate_instruction(&instruction).expect("AESEMC SVE should validate");
}

#[test]
fn aesemc_sve_uses_svespecial() {
    let instruction = aesemc_sve_instruction();

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
        panic!("expected SVESpecial for AESEMC SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::AESMixColumns);
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
