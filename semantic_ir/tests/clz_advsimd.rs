/// CLZ (vector) - Count leading zero bits (vector)
/// Stanford source: arm64_md_clean/clz_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       count = CountLeadingZeroBits(operand[e*esize]);
///       result[e*esize] = count[esize-1:0];
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn clz_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;  // 32-bit elements

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Clz,
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
            mnemonic: "CLZ".to_string(),
            encoding: Some("Count leading zero bits (vector)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "clz_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn clz_advsimd_validates() {
    let instruction = clz_advsimd_instruction();
    validate_instruction(&instruction).expect("CLZ (vector) should validate");
}

#[test]
fn clz_advsimd_uses_vector_map_clz() {
    let instruction = clz_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for CLZ");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Clz);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}