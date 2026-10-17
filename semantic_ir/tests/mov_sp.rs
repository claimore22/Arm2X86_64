/// MOV (to/from SP): alias of ADD (immediate) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/mov_add_addsub_imm.md
///
/// Alias of ADD (immediate) with imm = 0. Both the source and destination
/// registers are SP-aware: when the field is 31 the stack pointer is used
/// instead of XZR.
///
/// Stanford (arm64_md_clean/mov_add_addsub_imm.md), alias condition:
///   operand1 = if n == 31 then SP{datasize}() else X{datasize}(n);
///   X{datasize}(d) = result;  // d == 31 writes SP for this alias
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

/// Example: MOV Xd(=1), SP — i.e. n == 31 (source is SP), d == 1.
fn mov_sp_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    // operand1 = if n == 31 then SP{64}() else X{64}(n)  (n == 31 here)
    let n_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };
    let src = Expr::Select {
        condition: Box::new(n_is_sp),
        when_true: Box::new(Expr::ReadSP { width: datasize }),
        when_false: Box::new(Expr::ReadReg {
            reg: Reg::X(31.min(30)),
            width: datasize, // Match datasize for Select branch validation
        }),
        ty: ValueType::Bits(Width::Fixed(datasize)),
    };

    // result = operand1 + 0
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(src),
        right: Box::new(Expr::Const {
            value: 0,
            ty: datasize,
        }),
        ty: ValueType::Bits(Width::Fixed(datasize)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (to/from SP): alias of ADD (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_add_addsub_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

/// Example: MOV SP, Xn — destination is SP (d == 31), n == 2.
fn mov_to_sp_instruction() -> Instruction {
    let datasize = BitWidth::W64;

    let src = Expr::ReadReg {
        reg: Reg::X(2),
        width: datasize,
    };
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(src),
        right: Box::new(Expr::Const {
            value: 0,
            ty: datasize,
        }),
        ty: ValueType::Bits(Width::Fixed(datasize)),
    };

    // d == 31 writes SP (not XZR) for the to-SP alias.
    let d_is_sp = Expr::Binary {
        op: semantic_ir::BinOp::Eq,
        left: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        right: Box::new(Expr::Const {
            value: 31,
            ty: BitWidth::W32,
        }),
        ty: ValueType::Bool,
    };

    let semantics = vec![Stmt::If {
        condition: d_is_sp,
        then_body: vec![Stmt::WriteSP {
            value: result.clone(),
            width: Width::Fixed(datasize),
        }],
        else_body: vec![Stmt::WriteReg {
            reg: Reg::X(1),
            value: result,
        }],
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "MOV".to_string(),
            encoding: Some("MOV (to/from SP): alias of ADD (immediate)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "mov_add_addsub_imm.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn mov_sp_validates() {
    validate_instruction(&mov_sp_instruction()).expect("MOV from SP should validate");
    validate_instruction(&mov_to_sp_instruction()).expect("MOV to SP should validate");
}

#[test]
fn mov_sp_selects_sp_source() {
    let instruction = mov_sp_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Binary { left, .. } = value else {
        panic!("expected Add");
    };

    // Source must select between SP and Xn on n == 31.
    fn has_read_sp(expr: &Expr) -> bool {
        match expr {
            Expr::ReadSP { .. } => true,
            Expr::Select {
                condition,
                when_true,
                when_false,
..
            } => {
                has_read_sp(condition) || has_read_sp(when_true) || has_read_sp(when_false)
            }
            Expr::Binary { left, right, .. } => has_read_sp(left) || has_read_sp(right),
            Expr::Unary { value, .. } => has_read_sp(value),
            _ => false,
        }
    }
    assert!(has_read_sp(left), "source must be able to read SP");
}

#[test]
fn mov_sp_selects_sp_destination() {
    let instruction = mov_to_sp_instruction();
    let Stmt::If {
        then_body,
        else_body,
..
    } = &instruction.semantics[0]
    else {
        panic!("expected If for SP/X destination select");
    };
    assert!(matches!(then_body[0], Stmt::WriteSP { .. }));
    assert!(matches!(else_body[0], Stmt::WriteReg { .. }));
}

#[test]
fn mov_sp_uses_alias_semantics() {
    let instruction = mov_sp_instruction();
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(matches!(
        value,
        Expr::Binary {
            op: semantic_ir::BinOp::Add,
..
        }
    ));
}
