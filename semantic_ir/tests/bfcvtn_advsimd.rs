/// BFCVTN, BFCVTN2 - Single-precision convert to BFloat16 (vector)
/// Stanford source: arm64_md_clean/bfcvtn_advsimd.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn bfcvtn_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let input_esize = BitWidth::W32;  // 32-bit float input elements
    let output_esize = BitWidth::W16; // 16-bit BF16 output elements

    let operand = Expr::ReadReg {
        reg: Reg::V(1),
        width: datasize, // 128-bit source
    };

    // BFCVTN/BFCVTN2: Single-precision convert to BFloat16 (vector)
    // This is a narrowing conversion: 128-bit (4x32-bit) -> 64-bit (4x16-bit)
    // For unary narrowing ops like FCVTN, we use the same operand for both left and right
    let result = Expr::VectorNarrowing {
        op: semantic_ir::primitives::NarrowingOp::FCVTN,
        left: Box::new(operand.clone()),
        right: Box::new(operand),
        datasize,
        esize: input_esize,
        round: false,
        part: 0, // BFCVTN writes to lower half
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFCVTN".to_string(),
            encoding: Some("Single-precision convert to BFloat16 (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BF16".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvtn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfcvtn_advsimd_validates() {
    let instruction = bfcvtn_advsimd_instruction();
    validate_instruction(&instruction).expect("BFCVTN should validate");
}

#[test]
fn bfcvtn_advsimd_structure() {
    let instruction = bfcvtn_advsimd_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorNarrowing { op, left, right: _, datasize, esize, round, part } = value else {
        panic!("expected VectorNarrowing for BFCVTN");
    };

    assert_eq!(*op, semantic_ir::primitives::NarrowingOp::FCVTN);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(!*round);
    assert_eq!(*part, 0);
}