/// BFCVTN - BFloat16 convert to interleaved 8-bit floating-point
/// Stanford source: arm64_md_clean/bfcvtn_z8_mz2.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV 16;
///   let operand1 : bits(VL) = Z[n+0];
///   let operand2 : bits(VL) = Z[n+1];
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       let element1 : bits(16) = operand1[e*:16];
///       let element2 : bits(16) = operand2[e*:16];
///       result[(2*e + 0)*:8] = BFConvertFP8(element1, FPCR(), FPMR());
///       result[(2*e + 1)*:8] = BFConvertFP8(element2, FPCR(), FPMR());
///   end;
///   Z[VL](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn bfcvtn_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W16;   // Source element size (16-bit BFloat16)
    let result_esize = BitWidth::W8; // Result element size (8-bit float)

    let operand1 = Expr::ReadScalableVecReg { reg: 0, vl }; // Zn+0
    let operand2 = Expr::ReadScalableVecReg { reg: 1, vl }; // Zn+1

    // Use VectorInterleave for FP8 to BF16 conversion
    let result = Expr::VectorInterleave {
        op: semantic_ir::primitives::VectorInterleaveOp::FP8ToBF16Interleave,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: vl,
        esize,
        result_esize,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "BFCVTN".to_string(),
            encoding: Some("BFloat16 convert to interleaved 8-bit floating-point".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![
            "FEAT_SVE2".to_string(),
            "FEAT_SME2".to_string(),
            "FEAT_FP8".to_string(),
        ],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "bfcvtn_z8_mz2.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn bfcvtn_sve_unpredicated_validates() {
    let instruction = bfcvtn_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("BFCVTN SVE unpredicated should validate");
}

#[test]
fn bfcvtn_sve_unpredicated_structure() {
    let instruction = bfcvtn_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::VectorInterleave { op, left, right, datasize, esize, result_esize } = value else {
        panic!("expected VectorInterleave for BFCVTN");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorInterleaveOp::FP8ToBF16Interleave);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W16);
    assert_eq!(*result_esize, BitWidth::W8);
    assert!(matches!(left.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
    assert!(matches!(right.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
}