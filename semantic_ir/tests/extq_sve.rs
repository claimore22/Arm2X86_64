/// EXTQ (SVE) - Extract vector segment from each pair of quadword vector segments
/// Stanford source: arm64_md_clean/extq_z_zi.md
///
/// Stanford pseudocode:
///   let dn : integer = UInt(Zdn);
///   let m : integer = UInt(Zm);
///   let position : integer = UInt(imm4) << 3;
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let segments : integer = VL DIV 128;
///   let operand1 : bits(VL) = Z[dn];
///   let operand2 : bits(VL) = Z[m];
///   var result : bits(VL);
///   for s = 0 to segments-1 do
///       let concat : bits(256) = operand2[s*:128] :: operand1[s*:128];
///       result[s*:128] = concat[position+127:position];
///   end;
///   Z[VL](dn) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn extq_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let dn = 0u8; // Zdn
    let m = 1u8; // Zm

    let operand1 = Expr::ReadScalableVecReg { reg: dn, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: m, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Ext,
        predicate: None,
        operands: vec![Box::new(operand1), Box::new(operand2)],
        vl,
        esize: Some(BitWidth::W8),
        imm: Some(32), // position in bytes
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "EXTQ".to_string(),
            encoding: Some("Extract vector segment from each pair of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "extq_z_zi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn extq_sve_validates() {
    let instruction = extq_sve_instruction();
    validate_instruction(&instruction).expect("EXTQ SVE should validate");
}

#[test]
fn extq_uses_special_op() {
    let instruction = extq_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, operands, .. } = value else {
        panic!("expected SVESpecial for EXTQ SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Ext);
    assert_eq!(operands.len(), 2);
    assert!(matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
    assert!(matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
}
