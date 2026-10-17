/// EXT - Extract vector from pair of vectors
/// Stanford source: arm64_md_clean/ext_advsimd.md
/// 
/// Stanford pseudocode:
///   let hi : bits(datasize) = V{}(m);
///   let lo : bits(datasize) = V{}(n);
///   let concat : bits(datasize*2) = hi :: lo;
///   V{datasize}(d) = concat[(position+datasize)-1:position];
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ext_advsimd_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W128;
    let position = 32u16;  // Byte position

    let hi = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let lo = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // EXT: extract from concatenated pair of vectors
    let result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::Ext,
        operands: vec![hi, lo],
        datasize, esize,
        imm: Some(position),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "EXT".to_string(),
            encoding: Some("Extract vector from pair of vectors".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ext_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ext_advsimd_validates() {
    let instruction = ext_advsimd_instruction();
    validate_instruction(&instruction).expect("EXT should validate");
}

#[test]
fn ext_advsimd_uses_vector_perm_ext() {
    let instruction = ext_advsimd_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPerm { op, operands, datasize, esize, imm } = value else {
        panic!("expected VectorPerm for EXT");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::Ext);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W128);
    assert_eq!(*imm, Some(32u16));
    assert_eq!(operands.len(), 2);
    assert!(matches!(&operands[0], Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(&operands[1], Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}