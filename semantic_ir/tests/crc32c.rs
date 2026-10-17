/// CRC32CB, CRC32CH, CRC32CW, CRC32CX (CRC32C checksum) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/crc32c.md
///
/// Wd = CRC32(Wn accumulator, Xm input of `size` bits) using polynomial 0x1EDC6F41.
/// The second source operand can be 8 (B), 16 (H), 32 (W), or 64 (X) bits.
/// Requires FEAT_CRC32.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, validate_instruction,
};

fn crc32c_instruction(suffix: &str, data_width: BitWidth, src_reg: Reg) -> Instruction {
    let rd = Reg::W(1); // destination (always 32-bit)
    let rn = Reg::W(2); // accumulator (always 32-bit)

    // Stanford: acc = X(n)[31:0] (32-bit accumulator)
    let acc = Expr::ReadReg {
        reg: rn,
        width: BitWidth::W32,
    };

    // Stanford: val = X(m)[size-1:0]
    let val = Expr::ReadReg {
        reg: src_reg,
        width: data_width,
    };

    // Opaque CRC32 op (poly 0x1EDC6F41)
    let crc = Expr::Crc32 {
        acc: Box::new(acc),
        value: Box::new(val),
        data_width,
        crc32c: true,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: rd,
        value: crc,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: format!("CRC32C{suffix}"),
            encoding: Some("CRC32C checksum".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_CRC32".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "crc32c.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn crc32cb_validates() {
    let instruction = crc32c_instruction("B", BitWidth::W8, Reg::W(3));
    validate_instruction(&instruction).expect("CRC32CB should validate");
}

#[test]
fn crc32ch_validates() {
    let instruction = crc32c_instruction("H", BitWidth::W16, Reg::W(3));
    validate_instruction(&instruction).expect("CRC32CH should validate");
}

#[test]
fn crc32cw_validates() {
    let instruction = crc32c_instruction("W", BitWidth::W32, Reg::W(3));
    validate_instruction(&instruction).expect("CRC32CW should validate");
}

#[test]
fn crc32cx_validates() {
    let instruction = crc32c_instruction("X", BitWidth::W64, Reg::X(3));
    validate_instruction(&instruction).expect("CRC32CX should validate");
}

#[test]
fn crc32c_requires_feat_crc32() {
    let instruction = crc32c_instruction("B", BitWidth::W8, Reg::W(3));
    assert!(instruction.required_features.contains(&"FEAT_CRC32".to_string()));
}

#[test]
fn crc32c_rejects_value_width_mismatch() {
    // data_width declared as 32 but the value read is only 8-bit: must fail validation
    let mut instruction = crc32c_instruction("W", BitWidth::W32, Reg::W(3));
    instruction.semantics = vec![Stmt::WriteReg {
        reg: Reg::W(1),
        value: Expr::Crc32 {
            acc: Box::new(Expr::ReadReg {
                reg: Reg::W(2),
                width: BitWidth::W32,
            }),
            value: Box::new(Expr::ReadReg {
                reg: Reg::W(3),
                width: BitWidth::W8,
            }),
            data_width: BitWidth::W32,
            crc32c: true,
        },
    }];
    assert!(validate_instruction(&instruction).is_err());
}

#[test]
fn crc32c_reads_two_source_registers() {
    let instruction = crc32c_instruction("X", BitWidth::W64, Reg::X(3));

    let read_regs: Vec<_> = instruction
        .semantics
        .iter()
        .filter_map(|s| match s {
            Stmt::WriteReg {
                value: Expr::Crc32 { acc, value, .. },
..
            } => Some((acc.clone(), value.clone())),
            _ => None,
        })
        .collect();

    assert_eq!(read_regs.len(), 1);
    let (acc, val) = &read_regs[0];
    assert!(matches!(&**acc, Expr::ReadReg { reg: Reg::W(2), .. }));
    assert!(matches!(&**val, Expr::ReadReg { reg: Reg::X(3), .. }));
}
