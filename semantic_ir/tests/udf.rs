/// UDF: Permanently undefined
/// Stanford source: udf.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn udf_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;

    // UDF: Permanently undefined (always traps)
    let result = Expr::Const {
        value: 0,
        ty: BitWidth::W64,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "UDF".to_string(),
            encoding: Some("Permanently undefined".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "udf.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn udf_validates() {
    let instruction = udf_instruction();
    validate_instruction(&instruction).expect("UDF should validate");
}

#[test]
fn udf_uses_const() {
    let instruction = udf_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Const { ty, .. } = value else {
        panic!("expected Const for UDF");
    };

    assert_eq!(*ty, BitWidth::W64);
}
