/// CNT - Population count per byte
/// Stanford source: arm64_md_clean/cnt_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand : bits(datasize) = V{}(n);
///   var result : bits(datasize);
///   for e = 0 to elements-1 do
///       count = BitCount(operand[e*esize]);
///       result[e*esize] = count[esize-1:0];
///   end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn cnt_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W8;  // 8-bit elements for byte population count

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let result = Expr::VectorMap {
        op: semantic_ir::primitives::VectorUnaryOp::Cnt,
        operand: Box::new(operand),
        datasize: BitWidth::W128,
        esize: BitWidth::W8,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "CNT".to_string(),
            encoding: Some("Population count per byte".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cnt_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cnt_advsimd_validates() {
    let instruction = cnt_advsimd_instruction();
    validate_instruction(&instruction).expect("CNT should validate");
}

#[test]
fn cnt_advsimd_uses_vector_map_cnt() {
    let instruction = cnt_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorMap { op, operand, datasize, esize } = value else {
        panic!("expected VectorMap for CNT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorUnaryOp::Cnt);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W8);
    assert!(matches!(operand.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}