/// BFMUL (indexed) - BFloat16 multiply by indexed element (SVE)
/// Stanford source: arm64_md_clean/bfmul_z_zzi.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bfmul_sve_indexed_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W16;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl }; // Zm
    let destination = Expr::ReadScalableVecReg { reg: 0, vl }; // Zd

    // BFMUL (indexed): multiply by indexed element within each 128-bit segment
    // Uses index 0 for this test
    let index = Expr::Const { value: 0, ty: BitWidth::W3 };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::BfmulIndexed,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand1), Box::new(operand2), Box::new(index)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFMUL (indexed)".to_string(),
            encoding: Some("BFloat16 multiply by indexed element".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE_B16B16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfmul_z_zzi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfmul_sve_indexed_validates() {
    let instruction = bfmul_sve_indexed_instruction();
    validate_instruction(&instruction).expect("BFMUL (indexed) SVE should validate");
}

#[test]
fn bfmul_sve_indexed_structure() {
    let instruction = bfmul_sve_indexed_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, imm } = value else {
        panic!("expected SVESpecial for BFMUL indexed");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::BfmulIndexed);
    matches!(&**predicate.as_ref().unwrap(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 3);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
    matches!(operands[1].as_ref(), Expr::ReadScalableVecReg { reg: 2, vl: BitWidth::W256 });
    matches!(operands[2].as_ref(), Expr::Const { value: 0, ty: BitWidth::W3 });
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize.as_ref().unwrap(), BitWidth::W16);
}