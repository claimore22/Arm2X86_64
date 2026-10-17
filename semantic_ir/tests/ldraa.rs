/// LDRAA, LDRAB (Load register, with pointer authentication) semantics are
/// Stanford source: arm64_md_clean/ldra.md
/// representable in the IR.
///
/// data = Mem[X(n) + offset]; X(t) = AuthIA/AuthIB(data, address,
/// auth_combined=false). Requires FEAT_PAuth.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ldraa_instruction(key_a: bool) -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base.clone()),
        right: Box::new(Expr::Const { value: 16, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };

    let data = Expr::MemRead {
        address: Box::new(address.clone()),
        size: MemSize::Bits64,
    };

    let modifier = Expr::ReadSP {
        width: BitWidth::W64,
    };

    let authed = if key_a {
        Expr::AuthIA {
            address: Box::new(data),
            modifier: Box::new(modifier),
            auth_combined: Box::new(Expr::Bool(false)),
        }
    } else {
        Expr::AuthIB {
            address: Box::new(data),
            modifier: Box::new(modifier),
            auth_combined: Box::new(Expr::Bool(false)),
        }
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: authed,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: if key_a { "LDRAA" } else { "LDRAB" }.to_string(),
            encoding: Some("Load register, with pointer authentication".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_PAuth".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldra.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldraa_validates() {
    validate_instruction(&ldraa_instruction(true)).expect("LDRAA should validate");
}

#[test]
fn ldrab_validates() {
    validate_instruction(&ldraa_instruction(false)).expect("LDRAB should validate");
}

#[test]
fn ldraa_requires_pauth() {
    assert!(ldraa_instruction(true)
        .required_features
        .contains(&"FEAT_PAuth".to_string()));
}

#[test]
fn ldraa_authenticates_loaded_address() {
    let instruction = ldraa_instruction(true);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::WriteReg {
            value: Expr::AuthIA { .. },
..
        }
    ));
    let instruction = ldraa_instruction(false);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::WriteReg {
            value: Expr::AuthIB { .. },
..
        }
    ));
}
