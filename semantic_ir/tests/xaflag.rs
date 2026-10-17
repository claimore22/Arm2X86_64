/// XAFLAG (Convert floating-point condition flags from external format to Arm
/// Stanford source: arm64_md_clean/xaflag.md
/// format) semantics are representable in the IR.
///
/// Converts the FP-compatible flag format in NZCV to the Arm format
/// (bit manipulation per FEAT_FlagM pseudocode). Modeled as WriteFlag ops from
/// the current flags (placeholder constant composition).
/// Requires FEAT_FlagM.
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, Provenance, Stmt, validate_instruction,
};

fn xaflag_instruction() -> Instruction {
    // nzcv_arm built from current flags (mask can be any immediate)
    let nzcv = Expr::Const {
        value: 0,
        ty: BitWidth::W4,
    };
    let mut semantics = Vec::new();
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
            mnemonic: "XAFLAG".to_string(),
            encoding: Some("Convert FP condition flags to Arm format".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_FlagM".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "xaflag.md".to_string(),
            source_sha256: "t".to_string(),
            clean_sha256: "t".to_string(),
        },
    }
}

#[test]
fn xaflag_validates() {
    validate_instruction(&xaflag_instruction()).expect("XAFLAG should validate");
}

#[test]
fn xaflag_writes_all_flags() {
    let instruction = xaflag_instruction();
    let n = instruction
        .semantics
        .iter()
        .filter(|s| matches!(s, Stmt::WriteFlag { .. }))
        .count();
    assert_eq!(n, 4);
}
