/// LDATXR (Load-acquire unprivileged exclusive register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldatxr.md
///
/// Unprivileged exclusive load; compare value marked for exclusive monitor.
/// Modeled as an exclusive-monitor placeholder Let plus MemRead with
/// ZeroExtend to the destination register (pair: two loads for LDAXP/LDAXPD).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldatxr_instruction(mnemonic: &str, size: MemSize, pair: bool) -> Instruction {
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let mut semantics = vec![Stmt::Let {
        name: "set_exclusive_monitor".to_string(),
        value: Expr::Const { value: 1, ty: BitWidth::W8 },
    }];

    let width = size.width();
    let data = Expr::MemRead {
        address: Box::new(address),
        size,
    };

    let v0 = if width != BitWidth::W64 {
        Expr::ZeroExtend { value: Box::new(data.clone()), to: BitWidth::W64 }
    } else {
        data.clone()
    };
    semantics.push(Stmt::WriteReg { reg: Reg::X(0), value: v0 });

    if pair {
        semantics.push(Stmt::WriteReg {
            reg: Reg::X(2),
            value: if width != BitWidth::W64 {
                Expr::ZeroExtend { value: Box::new(data), to: BitWidth::W64 }
            } else {
                data
            },
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-acquire unprivileged exclusive register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldatxr.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldatxr_validates() {
    validate_instruction(&ldatxr_instruction("LDATXR", MemSize::Bits64, false))
        .expect("LDATXR should validate");
}

#[test]
fn ldatxr_sets_exclusive_monitor() {
    let instruction = ldatxr_instruction("LDATXR", MemSize::Bits64, false);
    assert!(matches!(
        &instruction.semantics[0],
        Stmt::Let { name, .. } if name == "set_exclusive_monitor"
    ));
}

#[test]
fn ldatxr_writes_destination() {
    let instruction = ldatxr_instruction("LDATXR", MemSize::Bits64, false);
    let n_writes = instruction
        .semantics
        .iter()
        .filter(|s| matches!(s, Stmt::WriteReg { .. }))
        .count();
    assert_eq!(n_writes, if false { 2 } else { 1 });
}
