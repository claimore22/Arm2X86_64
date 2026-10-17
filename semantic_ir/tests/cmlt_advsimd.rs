/// CMLT (zero) - Compare signed less than zero (vector)
/// Stanford source: arm64_md_clean/cmlt_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       element = SInt(operand[e*esize]);
///       result[e*esize] = if element < 0 then Ones{esize} else Zeros{esize};
///   end;
///   V{datasize}(d) = result.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn cmlt_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::CMLTZero,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W32,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CMLT".to_string(),
            encoding: Some("Compare signed less than zero (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cmlt_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cmlt_advsimd_validates() {
    let instruction = cmlt_advsimd_instruction();
    validate_instruction(&instruction).expect("CMLT should validate");
}

#[test]
fn cmlt_advsimd_uses_vector_map_cmlt_zero() {
    let instruction = cmlt_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for CMLTZero");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::CMLTZero);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}