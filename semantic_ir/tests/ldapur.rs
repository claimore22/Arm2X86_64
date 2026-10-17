/// LDAPUR, LDAPURB, LDAPURH, LDAPURSB, LDAPURSH, LDAPURSW (Load-acquire RCpc register, unscaled) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldapur_gen.md
///
/// Unscaled signed 9-bit immediate offset is added to the base address.
/// Modeled as MemRead on the address in Xn (+ imm9 for unscaled, folded into
/// the address expression), with ZeroExtend/SignExtend to the destination.
/// Feature: FEAT_LRCPC2
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldapur_instruction(mnemonic: &str, size: MemSize, signed: bool) -> Instruction {
    let width = size.width();
    let reg_width = if width == BitWidth::W64 { BitWidth::W64 } else { BitWidth::W32 };

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

        // address = X(n) + imm9 (unscaled)
    let address = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base),
        right: Box::new(Expr::Const { value: 8, ty: BitWidth::W64 }),
        ty: semantic_ir::ValueType::Bits(semantic_ir::Width::Fixed(BitWidth::W64)),
    };

    let data = Expr::MemRead {
        address: Box::new(address),
        size,
    };

    // extend to destination register width (64-bit for signed-to-X, else 32/64)
    let value = if signed {
        Expr::SignExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        }
    } else if width != BitWidth::W64 {
        Expr::ZeroExtend {
            value: Box::new(data),
            to: BitWidth::W64,
        }
    } else {
        data
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value,
    }];

    let _ = reg_width;

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load-acquire RCpc register (unscaled)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldapur_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldapur_validate() {    validate_instruction(&ldapur_instruction("LDAPUR", MemSize::Bits64, false))
        .expect("LDAPUR should validate");    validate_instruction(&ldapur_instruction("LDAPURB", MemSize::Bits8, false))
        .expect("LDAPURB should validate");    validate_instruction(&ldapur_instruction("LDAPURH", MemSize::Bits16, false))
        .expect("LDAPURH should validate");    validate_instruction(&ldapur_instruction("LDAPURSB", MemSize::Bits8, true))
        .expect("LDAPURSB should validate");    validate_instruction(&ldapur_instruction("LDAPURSH", MemSize::Bits16, true))
        .expect("LDAPURSH should validate");    validate_instruction(&ldapur_instruction("LDAPURSW", MemSize::Bits32, true))
        .expect("LDAPURSW should validate");}

#[test]
fn ldapur_requires_feature() {
    assert!(ldapur_instruction("LDAPUR", MemSize::Bits64, false)
        .required_features
        .contains(&"FEAT_LRCPC2".to_string()));
}

#[test]
fn ldapur_reads_memory() {
    let instruction = ldapur_instruction("LDAPUR", MemSize::Bits64, false);

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            Expr::Binary { left, right, .. } => has_mem_read(left) || has_mem_read(right),
            _ => false,
        }
    }

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    assert!(has_mem_read(value));
}
