/// FMOV (vector, immediate): Floating-point move immediate (vector)
/// Stanford source: fmov_vector_imm.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn fmov_vector_immediate_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    // FMOV (vector, immediate): Move immediate value to all elements of vector
    let result = Expr::Const {
        value: 1.0f32.to_bits() as u128, // Example: 1.0
        ty: BitWidth::W128,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMOV (vector, immediate)".to_string(),
            encoding: Some("Floating-point move immediate (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmov_vector_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmov_vector_immediate_validates() {
    let instruction = fmov_vector_immediate_instruction();
    validate_instruction(&instruction).expect("FMOV (vector, immediate) should validate");
}

#[test]
fn fmov_vector_immediate_uses_const() {
    let instruction = fmov_vector_immediate_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Const { ty, .. } = value else {
        panic!("expected Const for FMOV (vector, immediate)");
    };

    assert_eq!(*ty, BitWidth::W128);
}