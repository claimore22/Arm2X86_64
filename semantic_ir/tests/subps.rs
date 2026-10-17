/// SUBPS (Subtract pointer, setting flags) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/subps.md
///
/// Xd = Xn - Xm with NZCV set from the result (66-bit subtractor per FEAT_CPA
/// pointer checks). Modeled with AddWithCarry(Xn, NOT(Xm), C=1); the result
/// and nzcv tuple elements are written to Xd and PSTATE flags.
/// Requires FEAT_CPA (FEAT_MTE for tagged pointers).
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn subps_instruction() -> Instruction {
    let sum = Expr::AddWithCarry {
        left: Box::new(Expr::ReadReg {
            reg: Reg::X(2),
            width: BitWidth::W64,
        }),
        right: Box::new(Expr::Unary {
            op: semantic_ir::UnaryOp::Not,
            value: Box::new(Expr::ReadReg {
                reg: Reg::X(3),
                width: BitWidth::W64,
            }),
            ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
        }),
        carry: Box::new(Expr::Bool(true)),
        width: BitWidth::W64,
    };

    let result = Expr::TupleElement {
        value: Box::new(sum.clone()),
        index: 0,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let nzcv = Expr::TupleElement {
        value: Box::new(sum),
        index: 1,
        ty: ValueType::Bits(Width::Fixed(BitWidth::W4)),
    };

    let mut semantics = vec![Stmt::WriteReg {
        reg: Reg::X(1),
        value: result,
    }];

    for (flag, bit) in [(Flag::N, 3u16), (Flag::Z, 2), (Flag::C, 1), (Flag::V, 0)] {
        semantics.push(Stmt::WriteFlag {
            flag,
            value: Expr::BitTest {
                value: Box::new(nzcv.clone()),
                bit,
            },
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: "SUBPS".to_string(),
            encoding: Some("Subtract pointer, setting flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CPA".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "subps.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn subps_validates() {
    validate_instruction(&subps_instruction()).expect("SUBPS should validate");
}

#[test]
fn subps_requires_cpa() {
    assert!(subps_instruction()
        .required_features
        .contains(&"FEAT_CPA".to_string()));
}

#[test]
fn subps_writes_nzcv() {
    let instruction = subps_instruction();
    let flags: Vec<_> = instruction
        .semantics
        .iter()
        .filter_map(|s| match s {
            Stmt::WriteFlag { flag, .. } => Some(*flag),
            _ => None,
        })
        .collect();
    assert_eq!(flags.len(), 4);
    assert!(flags.contains(&Flag::N));
    assert!(flags.contains(&Flag::Z));
    assert!(flags.contains(&Flag::C));
    assert!(flags.contains(&Flag::V));
}

#[test]
fn subps_writes_result() {
    let instruction = subps_instruction();
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg {
            reg: Reg::X(1),
            value: Expr::TupleElement { index: 0, .. }
        }
    ));
}
