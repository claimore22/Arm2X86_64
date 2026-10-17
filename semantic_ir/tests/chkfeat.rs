/// CHKFEAT semantics are representable in the IR.
/// Stanford source: arm64_md_clean/chkfeat.md
///
/// CHKFEAT (Carm64_md_cleanchecks the status of features.
/// Stanford (arm64_md_no_nav_clean/chkfeat.md):
///   X[16] = AArch64_ChkFeat(X[16])
///
/// Requires FEAT_CHK.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn chkfeat_instruction() -> Instruction {
    // CHKFEAT: X[16] = AArch64_ChkFeat(X[16])
    let x16 = Expr::ReadReg {
        reg: Reg::X(16),
        width: BitWidth::W64,
    };

    let chkfeat_result = Expr::ChkFeat {
        value: Box::new(x16),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(16),
        value: chkfeat_result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CHKFEAT".to_string(),
            encoding: Some("Check feature status".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CHK".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "chkfeat.md".to_string(),
            source_sha256: "951f8171db06fb4565739e8123920f285a8b560982151a7899593b6a1d3f0c1e".to_string(),
            clean_sha256: "1091321da97bcf2d49b6862fa01bd949864212c0b4a8d739d14a774c8ea6d228".to_string(),
        },
    }
}

#[test]
fn chkfeat_validates() {
    let instruction = chkfeat_instruction();

    validate_instruction(&instruction).expect("CHKFEAT should validate");
}

#[test]
fn chkfeat_requires_chk_feature() {
    let instruction = chkfeat_instruction();
    assert_eq!(instruction.required_features, vec!["FEAT_CHK".to_string()]);
}

#[test]
fn chkfeat_reads_and_writes_x16() {
    let instruction = chkfeat_instruction();

    assert_eq!(instruction.semantics.len(), 1);

    let Stmt::WriteReg { reg, value } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    assert_eq!(*reg, Reg::X(16));

    // Value should be ChkFeat(X[16])
    let Expr::ChkFeat { value } = value else {
        panic!("expected ChkFeat");
    };

    assert!(matches!(value.as_ref(), Expr::ReadReg { reg: Reg::X(16), width: BitWidth::W64 }));
}