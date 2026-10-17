/// DUP (element) - Duplicate vector element to vector or scalar
/// Stanford source: arm64_md_clean/dup_advsimd_elt.md
/// 
/// Stanford pseudocode:
///   let operand : bits(idxdsize) = V{}(n);
///   var result : bits(datasize);
///   element = operand[index*esize];
///   for e = 0 to elements-1 do result[e*esize] = element; end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn dup_advsimd_elt_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W32;
    let index = 2u8;  // Element index

    let operand = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // DUP: extract element at index and duplicate to all elements
    let result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::Dup,
        operands: vec![operand],
        datasize, esize,
        imm: Some(index as u16),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP".to_string(),
            encoding: Some("Duplicate vector element to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_advsimd_elt.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_advsimd_elt_validates() {
    let instruction = dup_advsimd_elt_instruction();
    validate_instruction(&instruction).expect("DUP (element) should validate");
}

#[test]
fn dup_advsimd_elt_uses_vector_perm_dup() {
    let instruction = dup_advsimd_elt_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPerm { op, operands, datasize, esize, imm } = value else {
        panic!("expected VectorPerm for DUP");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::Dup);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W32);
    assert_eq!(*imm, Some(2u16));
    assert_eq!(operands.len(), 1);
    assert!(matches!(&operands[0], Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}