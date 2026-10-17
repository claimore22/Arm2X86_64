/// BTI (Branch Target Identification) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/bti.md
/// Stanford (arm64_md_no_nav_clean/bti.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_BTI) then EndOfDecode(Decode_NOP); end;
///     SetBTypeCompatible(BTypeCompatible_BTI(op2[2:1]));
///   Operation:
///     SetBTypeNext('00');
///
/// Variants for <targets> (encoded in op2[2:1]):
///   - 0b00: absent (BTI)
///   - 0b01: 'c'    (BTI c)
///   - 0b10: 'j'    (BTI j)
///   - 0b11: 'jc'   (BTI jc)
///
/// No IR extension: BType / BTypeNext tracking is omitted in the IR (precedent
/// in BLR, BR, BRK). The indirection target requirement is captured in decode
/// and preserved in semantics via `Stmt::Let(targets: W2)`.
/// Requires `FEAT_BTI`.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    Provenance, Stmt, validate_instruction,
};

fn bti_instruction(targets: u128) -> Instruction {
    let encoding_desc = match targets {
        0b00 => "Branch target identification",
        0b01 => "Branch target identification (c)",
        0b10 => "Branch target identification (j)",
        0b11 => "Branch target identification (jc)",
        _ => "Branch target identification",
    };

    let target_width = BitWidth::new(2).expect("2-bit width should be valid");

    Instruction {
        id: InstructionId {
            mnemonic: "BTI".to_string(),
            encoding: Some(encoding_desc.to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_BTI".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![DecodeStmt::Let {
                name: "targets".to_string(),
                value: DecodeExpr::UInt("op2<2:1>".to_string()),
            }],
        }),
        semantics: vec![Stmt::Let {
            name: "targets".to_string(),
            value: Expr::Const {
                value: targets,
                ty: target_width,
            },
        }],
        provenance: Provenance {
            source_path: "bti.md".to_string(),
            source_sha256:
                "64e6da3aae440424dda48a9655be6eaa88a7e74c1dced77437c2635b6b03d101".to_string(),
            clean_sha256:
                "9b45716bcc40b49cd5ff818823187684bbdd06254d9b01dc3fa9b40a6d6c5c47".to_string(),
        },
    }
}

#[test]
fn bti_all_variants_validate() {
    for targets in [0b00, 0b01, 0b10, 0b11] {
        let ins = bti_instruction(targets);
        validate_instruction(&ins).expect("BTI variant should validate");
    }
}

#[test]
fn bti_requires_feat_bti() {
    let ins = bti_instruction(0b00);
    assert_eq!(ins.required_features, vec!["FEAT_BTI".to_string()]);
}

#[test]
fn bti_preserves_targets_operand() {
    for targets in [0b00, 0b01, 0b10, 0b11] {
        let ins = bti_instruction(targets);
        assert_eq!(ins.semantics.len(), 1);
        let Stmt::Let { name, value } = &ins.semantics[0] else {
            panic!("expected Let targets");
        };
        assert_eq!(name, "targets");
        let Expr::Const { value: v, ty } = value else {
            panic!("expected Const for targets");
        };
        assert_eq!(*v, targets);
        assert_eq!(ty.bits(), 2);
    }
}

#[test]
fn bti_decode_binds_targets() {
    let ins = bti_instruction(0b01);
    let decode = ins.decode.expect("BTI should have decode spec");
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "targets"
    )));
}
