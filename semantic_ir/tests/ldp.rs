/// LDP (Load pair of registers) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/ldp_gen.md
///
/// Stanford (arm64_md_clean/ldp_gen.md), signed offset class, little-endian:
///   address = (if n == 31 then SP[64] else X[n,64]) + offset;
///   data    = Mem{2*datasize}(address);
///   data1   = data[datasize-1:0];        // X[t]
///   data2   = data[2*datasize-1:datasize]; // X[t2]
///
/// Modeled as two MemReads at address and address + dbytes (equivalent to the
/// wide read + extract on a little-endian target).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, MemSize, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn ldp_instruction(mnemonic: &str, size: MemSize) -> Instruction {
    let width = size.width();
    let dbytes = (size.width().bits() / 8) as u128;

    let base = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    // address2 = address + dbytes
    let addr2 = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(base.clone()),
        right: Box::new(Expr::Const {
            value: dbytes,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let data0 = Expr::MemRead {
        address: Box::new(base),
        size,
    };
    let data1 = Expr::MemRead {
        address: Box::new(addr2),
        size,
    };

    let v0 = if width != BitWidth::W64 {
        Expr::ZeroExtend {
            value: Box::new(data0),
            to: BitWidth::W64,
        }
    } else {
        data0
    };
    let v1 = if width != BitWidth::W64 {
        Expr::ZeroExtend {
            value: Box::new(data1),
            to: BitWidth::W64,
        }
    } else {
        data1
    };
    let semantics = vec![
        Stmt::WriteReg {
            reg: Reg::X(0),
            value: v0,
        },
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: v1,
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Load pair of registers".to_string()),
        },
        operands: Vec::new(),
        required_features: vec![],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ldp_gen.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ldp_validates() {
    validate_instruction(&ldp_instruction("LDP", MemSize::Bits64))
        .expect("LDP should validate");
    validate_instruction(&ldp_instruction("LDP", MemSize::Bits32))
        .expect("LDP 32-bit should validate");
}

#[test]
fn ldp_reads_memory() {
    let instruction = ldp_instruction("LDP", MemSize::Bits64);

    fn has_mem_read(expr: &Expr) -> bool {
        match expr {
            Expr::MemRead { .. } => true,
            Expr::ZeroExtend { value, .. } | Expr::SignExtend { value, .. } => has_mem_read(value),
            Expr::Binary { left, right, .. } => has_mem_read(left) || has_mem_read(right),
            Expr::ExtendReg { register, shift, .. } => has_mem_read(shift),
            _ => false,
        }
    }

    let reads: Vec<bool> = instruction
        .semantics
        .iter()
        .map(|s| match s {
            Stmt::WriteReg { value, .. } => has_mem_read(value),
            _ => false,
        })
        .collect();
    assert!(reads.iter().all(|r| *r), "both destinations read memory");
}

#[test]
fn ldp_second_load_is_offset_by_datasize() {
    let instruction = ldp_instruction("LDP", MemSize::Bits64);
    let Stmt::WriteReg { value, .. } = &instruction.semantics[1] else {
        panic!("expected WriteReg");
    };
    let Expr::MemRead { address, size } = value else {
        panic!("64-bit second load must be a raw MemRead");
    };
    assert_eq!(*size, MemSize::Bits64);

    // address2 = base + 8
    let Expr::Binary {
        op: semantic_ir::BinOp::Add,
        right,
..
    } = address.as_ref()
    else {
        panic!("second address must be base + dbytes");
    };
    assert!(matches!(
        right.as_ref(),
        Expr::Const {
            value: 8,
            ty: BitWidth::W64
        }
    ));
}

#[test]
fn ldp_writes_two_destinations() {
    let instruction = ldp_instruction("LDP", MemSize::Bits64);
    assert_eq!(instruction.semantics.len(), 2);
    assert!(matches!(
        instruction.semantics[0],
        Stmt::WriteReg { reg: Reg::X(0), .. }
    ));
    assert!(matches!(
        instruction.semantics[1],
        Stmt::WriteReg { reg: Reg::X(2), .. }
    ));
}
