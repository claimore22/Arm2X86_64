/// LD64B (Single-copy atomic 64-byte Load) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ld64b.md
///
/// Loads eight 64-bit doublewords atomically from a 64-byte aligned memory
/// location into consecutive registers: X(t+7)::...::Xt = Data[511:0].
/// Modeled as eight MemRead+WriteReg pairs at consecutive 8-byte offsets.
/// Requires FEAT_LS64.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ld64b_instruction() -> Instruction {
    let base = Expr::ReadReg {
        reg: Reg::X(2),
        width: BitWidth::W64,
    };

    let mut semantics = Vec::new();

    // for i = 0 to 7: X(t+i) = data[63+64*i : 64*i]
    for i in 0..8u8 {
        let addr = Expr::Binary {
            op: semantic_ir::BinOp::Add,
            left: Box::new(base.clone()),
            right: Box::new(Expr::Const {
                value: (i as u128) * 8,
                ty: BitWidth::W64,
            }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        };

        let value = Expr::MemRead {
            address: Box::new(addr),
            size: MemSize::Bits64,
        };

        semantics.push(Stmt::WriteReg {
            reg: Reg::X(1 + i), // t = 1 here; callee-chosen
            value,
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: "LD64B".to_string(),
            encoding: Some("Single-copy atomic 64-byte load".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LS64".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld64b.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld64b_validates() {
    let instruction = ld64b_instruction();
    validate_instruction(&instruction).expect("LD64B should validate");
}

#[test]
fn ld64b_requires_feat_ls64() {
    assert!(ld64b_instruction()
        .required_features
        .contains(&"FEAT_LS64".to_string()));
}

#[test]
fn ld64b_writes_eight_consecutive_registers() {
    let instruction = ld64b_instruction();

    assert_eq!(instruction.semantics.len(), 8);
    for (i, stmt) in instruction.semantics.iter().enumerate() {
        let Stmt::WriteReg { reg, value } = stmt else {
            panic!("expected WriteReg");
        };
        assert_eq!(*reg, Reg::X(1 + i as u8));
        assert!(matches!(value, Expr::MemRead { size: MemSize::Bits64, .. }));
    }
}
