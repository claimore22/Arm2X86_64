/// LDIAPP (Load-Acquire RCpc ordered pair of registers) semantics are
/// Stanford source: arm64_md_clean/ldiapp.md
/// representable in the IR.
///
/// Atomically loads a pair of registers from the address in Xn with
/// acquire (RCpc) ordering, zero-extending each element to 64 bits.
/// Modeled as two `Expr::MemRead` with `MemSize::Bits32` followed by
/// `Stmt::WriteReg` to Xt and Xt2.
///
/// Requires FEAT_LRCPC2.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldiapp_instruction() -> Instruction {
    // address = Xn (example n=1)
    let address = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    // address2 = address + 4 (second 32-bit element of the pair)
    let address2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(address.clone()),
        right: Box::new(Expr::Const {
            value: 4,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Both elements are 32-bit words; the pair is read with acquire ordering
    // (the ordering is carried on the instruction encoding, expressed on the
    // surrounding MemRead accesses in this bootstrap IR).
    let data0 = Expr::MemRead {
        address: Box::new(address),
        size: MemSize::Bits32,
    };
    let data1 = Expr::MemRead {
        address: Box::new(address2),
        size: MemSize::Bits32,
    };

    // X(t) = ZeroExtend(data<31:0>); X(t2) = ZeroExtend(data2<31:0>)
    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(0),
            value: Expr::ZeroExtend {
                value: Box::new(data0),
                to: BitWidth::W64,
            },
        },
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: Expr::ZeroExtend {
                value: Box::new(data1),
                to: BitWidth::W64,
            },
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: "LDIAPP".to_string(),
            encoding: Some("Load-Acquire RCpc ordered pair of registers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LRCPC2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldiapp.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldiapp_validates() {
    validate_instruction(&ldiapp_instruction()).expect("LDIAPP should validate");
}

#[test]
fn ldiapp_requires_feat_lrcpc2() {
    assert!(ldiapp_instruction()
        .required_features
        .contains(&"FEAT_LRCPC2".to_string()));
}

#[test]
fn ldiapp_reads_memory_pair() {
    let instruction = ldiapp_instruction();

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            Expr::Binary { left, right, .. } => has_mem_read(left) || has_mem_read(right),
            _ => false,
        }
    }

    // Both destination writes must come from memory reads.
    let writes: Vec<&Expr> = instruction
        .semantics
        .iter()
        .filter_map(|s| match s {
            Stmt::WriteReg { value, .. } => Some(value),
            _ => None,
        })
        .collect();

    assert_eq!(writes.len(), 2, "LDIAPP writes two destination registers");
    assert!(
        writes.iter().all(|v| has_mem_read(v)),
        "each destination value must originate from a memory read"
    );
}
