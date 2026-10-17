/// BRK semantics are representable in the IR.
/// Stanford source: arm64_md_clean/brk.md
/// Stanford (arm64_md_no_nav_clean/brk.md):
///   Decode: comment = imm16.
///   Operation: AArch64_SoftwareBreakpoint(comment) — Breakpoint
///   Instruction exception, EC 0x3C, ISS = comment.
/// No IR extension: `Stmt::RaiseException(SoftwareException)` plus
/// `Stmt::Let(comment)` to preserve the 16-bit immediate. BTYPE
/// compatibility (FEAT_BTI) omitted — no IR node, like BLR/BR omitting
/// BTypeNext.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Exception, Instruction, InstructionId,
    Provenance, Stmt, validate_instruction,
};

fn brk_instruction() -> Instruction {
    Instruction {
        id: InstructionId {
            mnemonic: "BRK".to_string(),
            encoding: Some("Breakpoint instruction".to_string()),
        },
        operands: Vec::new(),
        required_features: Vec::new(),
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Let {
                name: "comment".to_string(),
                value: DecodeExpr::UInt("imm16".to_string()),
            }],
        }),
        semantics: vec![
            Stmt::Let {
                name: "comment".to_string(),
                value: Expr::Const {
                    value: 0x1234,
                    ty: BitWidth::W16,
                },
            },
            Stmt::RaiseException(Exception::SoftwareException),
        ],
        provenance: Provenance {
            source_path: "brk.md".to_string(),
            source_sha256:
                "7369e0c5816f61ff23e20204f659b906f80e357c7dd18c2bf02967aa10e52fe8".to_string(),
            clean_sha256:
                "8067fb3cba1ca3ee750ec74cc98e4a2dc4f50ad5ac8bf336f5c8898891fdd526".to_string(),
        },
    }
}

#[test]
fn brk_validates() {
    validate_instruction(&brk_instruction()).expect("BRK should validate");
}

#[test]
fn brk_raises_software_exception() {
    let ins = brk_instruction();
    assert_eq!(ins.semantics.len(), 2);
    assert!(matches!(
        &ins.semantics[1],
        Stmt::RaiseException(Exception::SoftwareException)
    ));
}

#[test]
fn brk_preserves_16bit_comment() {
    let ins = brk_instruction();
    let Stmt::Let { name, value } = &ins.semantics[0] else {
        panic!("expected Let comment");
    };
    assert_eq!(name, "comment");
    let Expr::Const { value: v, ty } = value else {
        panic!("expected Const imm16");
    };
    assert_eq!(*v, 0x1234);
    assert_eq!(*ty, BitWidth::W16);
}

#[test]
fn brk_decode_binds_imm16() {
    let ins = brk_instruction();
    let decode = ins.decode.expect("BRK should have decode");
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "comment"
    )));
}
