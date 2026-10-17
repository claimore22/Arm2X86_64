/// CTERMEQ, CTERMNE - Compare and terminate loop
/// Stanford source: arm64_md_clean/ctermeq_rr.md
///
/// Stanford pseudocode:
///   let operand1 : bits(esize) = X[n];
///   let operand2 : bits(esize) = X[m];
///   let element1 : integer = UInt(operand1);
///   let element2 : integer = UInt(operand2);
///   var term : boolean;
///   case cmp_op of
///       when Cmp_EQ => term = element1 == element2;
///       when Cmp_NE => term = element1 != element2;
///   end;
///   if term then PSTATE.N = '1'; PSTATE.V = '0';
///   else PSTATE.N = '0'; PSTATE.V = (NOT PSTATE.C); end;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn create_cterm_instruction(mnemonic: &str, cmp_eq: bool) -> Instruction {
    let esize = BitWidth::W64; // 64-bit registers
    let n = 0u8;
    let m = 1u8;

    let operand1 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(n), width: esize };
    let operand2 = Expr::ReadReg { reg: semantic_ir::types::Reg::X(m), width: esize };

    // term = element1 == element2 (for CTERMEQ) or element1 != element2 (for CTERMNE)
    let cmp_op = if cmp_eq {
        semantic_ir::BinOp::Eq
    } else {
        semantic_ir::BinOp::Ne
    };

    let term = Expr::Binary {
        op: cmp_op,
        left: Box::new(operand1.clone()),
        right: Box::new(operand2.clone()),
        ty: semantic_ir::types::ValueType::Bool,
    };

    // if term then PSTATE.N = '1', PSTATE.V = '0'
    // else PSTATE.N = '0', PSTATE.V = (NOT PSTATE.C)
    let c_flag = Expr::ReadFlag { flag: semantic_ir::types::Flag::C };
    let not_c = Expr::Unary {
        op: semantic_ir::UnaryOp::Not,
        value: Box::new(c_flag),
        ty: semantic_ir::types::ValueType::Bool,
    };

    let n_flag = Expr::Select {
        condition: Box::new(term.clone()),
        when_true: Box::new(Expr::Bool(true)),
        when_false: Box::new(Expr::Bool(false)),
        ty: semantic_ir::types::ValueType::Bool,
    };

    let v_flag = Expr::Select {
        condition: Box::new(term),
        when_true: Box::new(Expr::Bool(false)),
        when_false: Box::new(not_c),
        ty: semantic_ir::types::ValueType::Bool,
    };

    let semantics = vec![
        Stmt::WriteFlag { flag: semantic_ir::types::Flag::N, value: n_flag },
        Stmt::WriteFlag { flag: semantic_ir::types::Flag::V, value: v_flag },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Compare and terminate loop".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ctermeq_rr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn ctermeq_sve_instruction() -> Instruction {
    create_cterm_instruction("CTERMEQ", true)
}

fn ctermne_sve_instruction() -> Instruction {
    create_cterm_instruction("CTERMNE", false)
}

#[test]
fn ctermeq_sve_validates() {
    let instruction = ctermeq_sve_instruction();
    validate_instruction(&instruction).expect("CTERMEQ SVE should validate");
}

#[test]
fn ctermeq_sve_sets_flags() {
    let instruction = ctermeq_sve_instruction();

    // Should have two WriteFlag statements (N and V)
    assert_eq!(instruction.semantics.len(), 2);

    let flags: Vec<_> = instruction.semantics.iter().filter_map(|s| {
        if let Stmt::WriteFlag { flag, .. } = s {
            Some(*flag)
        } else {
            None
        }
    }).collect();

    assert!(flags.contains(&semantic_ir::types::Flag::N));
    assert!(flags.contains(&semantic_ir::types::Flag::V));
}

#[test]
fn ctermne_sve_validates() {
    let instruction = ctermne_sve_instruction();
    validate_instruction(&instruction).expect("CTERMNE SVE should validate");
}

#[test]
fn ctermne_sve_uses_ne() {
    let instruction = ctermne_sve_instruction();

    let Stmt::WriteFlag { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteFlag");
    };

    let Expr::Select { condition, .. } = value else {
        panic!("expected Select for N flag");
    };

    let Expr::Binary { op, .. } = condition.as_ref() else {
        panic!("expected Binary for term");
    };

    assert_eq!(*op, semantic_ir::BinOp::Ne);
}