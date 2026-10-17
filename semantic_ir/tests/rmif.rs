/// RMIF (Rotate, mask insert flags) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/rmif.md
///
/// flags = RotateRight(Xn, imm6)[3:0]; under control of mask, the four bits
/// are inserted into PSTATE.{N,Z,C,V}. Requires FEAT_FlagM.
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn rmif_instruction() -> Instruction {
    let reg = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    // rotated = ROR(Xn, imm6)
    let rotated = Expr::Shift {
        kind: semantic_ir::ShiftKind::Ror,
        value: Box::new(reg),
        amount: Box::new(Expr::Const { value: 5, ty: BitWidth::W64 }),
        ty: BitWidth::W64,
    };

    // flags = rotated[3:0]
    let flags = Expr::Extract {
        value: Box::new(rotated),
        lsb: 0,
        width: BitWidth::W4,
    };

    // insert under mask into PSTATE.{N,Z,C,V}
    let mut semantics = Vec::new();
    for (flag, bit) in [(Flag::N, 3u16), (Flag::Z, 2), (Flag::C, 1), (Flag::V, 0)] {
        semantics.push(Stmt::WriteFlag {
            flag,
            value: Expr::BitTest {
                value: Box::new(flags.clone()),
                bit,
            },
        });
    }

    Instruction {
        id: InstructionId {
            mnemonic: "RMIF".to_string(),
            encoding: Some("Rotate, mask insert flags".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FlagM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rmif.md".to_string(),
            source_sha256: "t".to_string(),
            clean_sha256: "t".to_string(),
        },
    }
}

#[test]
fn rmif_validates() {
    validate_instruction(&rmif_instruction()).expect("RMIF should validate");
}

#[test]
fn rmif_writes_all_four_flags() {
    let instruction = rmif_instruction();
    let flags: Vec<_> = instruction
        .semantics
        .iter()
        .filter_map(|s| match s {
            Stmt::WriteFlag { flag, .. } => Some(*flag),
            _ => None,
        })
        .collect();
    assert_eq!(flags.len(), 4);
}

#[test]
fn rmif_requires_flagm() {
    assert!(rmif_instruction()
        .required_features
        .contains(&"FEAT_FlagM".to_string()));
}
