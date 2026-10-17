/// LDLAR, LDLARB, LDLARH (Load LOAcquire register) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldlar.md
///
/// Load-acquire (LOAcquire) register; word/doubleword or byte/halfword with zero-extension. Requires FEAT_LOR.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

fn ldlar_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();

        let address = Box::new(Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    let data = Expr::MemRead { address, size };

    let v = if width != BitWidth::W64 {
        Expr::ZeroExtend { value: Box::new(data), to: BitWidth::W64 }
    } else { data };
    let semantics = vec![Stmt::WriteReg { reg: Reg::X(0), value: v }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load LOAcquire register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LOR".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldlar.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldlar_validates() {
    validate_instruction(&ldlar_instruction("LDLAR", MemSize::Bits64))
        .expect("LDLAR should validate");
}

#[test]
fn ldlar_reads_memory() {
    let instruction = ldlar_instruction("LDLAR", MemSize::Bits64);

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            Expr::Binary { left, right, .. } => has_mem_read(left) || has_mem_read(right),
            Expr::ExtendReg { register, shift, .. } => has_mem_read(shift),
            _ => false,
        }
    }

    let found = instruction.semantics.iter().any(|s| match s {
        Stmt::WriteReg { value, .. } => has_mem_read(value),
        _ => false,
    });
    assert!(found, "expected a memory read");
}

#[test]
fn ldlar_writes_destination() {
    let instruction = ldlar_instruction("LDLAR", MemSize::Bits64);
    assert!(instruction
        .semantics
        .iter()
        .any(|s| matches!(s, Stmt::WriteReg { .. })));
}
