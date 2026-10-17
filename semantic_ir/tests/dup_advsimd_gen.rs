/// DUP (general) - Duplicate general-purpose register to vector
/// Stanford source: arm64_md_clean/dup_advsimd_gen.md
/// 
/// Stanford pseudocode:
///   let element : bits(esize) = X{}(n);
///   let elements : integer = datasize DIV esize;
///   var result : bits(datasize);
///   for e = 0 to elements-1 do result[e*esize] = element; end;
///   V{datasize}(d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn dup_advsimd_gen_instruction() -> Instruction {
    let datasize = BitWidth::W128;
    let esize = BitWidth::W64;  // 64-bit elements

    // Use a V register as source with full datasize (128-bit)
    // The Dup operation will extract the appropriate element and duplicate
    let element = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    // DUP: duplicate the source register to all elements
    let result = Expr::VectorPerm {
        op: semantic_ir::primitives::VectorPermOp::Dup,
        operands: vec![element],
        datasize, esize,
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "DUP".to_string(),
            encoding: Some("Duplicate general-purpose register to vector".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "dup_advsimd_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn dup_advsimd_gen_validates() {
    let instruction = dup_advsimd_gen_instruction();
    validate_instruction(&instruction).expect("DUP (general) should validate");
}

#[test]
fn dup_advsimd_gen_uses_vector_perm_dup() {
    let instruction = dup_advsimd_gen_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorPerm { op, operands, datasize, esize, imm } = value else {
        panic!("expected VectorPerm for DUP");
    };

    assert_eq!(*op, semantic_ir::primitives::VectorPermOp::Dup);
    assert_eq!(*datasize, BitWidth::W128);
    assert_eq!(*esize, BitWidth::W64);
    assert!(imm.is_none());
    assert_eq!(operands.len(), 1);
    assert!(matches!(&operands[0], Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
}