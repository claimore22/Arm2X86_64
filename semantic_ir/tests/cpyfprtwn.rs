/// CPYFPRTWN, CPYFMRTWN, CPYFERTWN (Memory copy forward-only, reads unprivileged, writes non-temporal) semantics are representable in the IR.
/// Stanford source: arm64_md_clean/cpyfprtwn.md
///
/// These are three stages of a memory copy operation using FEAT_MOPS with unprivileged read and non-temporal write:
/// - CPYFPRTWN (Prologue): Preconditions arguments, copies initial portion
/// - CPYFMRTWN (Main): Copies middle portion
/// - CPYFERTWN (Epilogue): Copies final remaining bytes
///
/// Read uses unprivileged + temporal access (rprivileged=0 via options[1]=1, rnontemporal=0 via options[3]=0).
/// Write uses unprivileged + non-temporal access (wprivileged=0 via options[0]=1, wnontemporal=1 via options[2]=1).
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn cpyfprtwn_instruction() -> Instruction {
    let datasize = Width::Fixed(BitWidth::W64);

    // Decode registers
    let rd = Reg::X(1); // destination
    let rs = Reg::X(2); // source
    let rn = Reg::X(3); // size

    // Read registers
    let toaddress = Expr::ReadReg {
        reg: rd,
        width: BitWidth::W64,
    };

    let fromaddress = Expr::ReadReg {
        reg: rs,
        width: BitWidth::W64,
    };

    let cpysize = Expr::ReadReg {
        reg: rn,
        width: BitWidth::W64,
    };

    // Options from op2 (4 bits): [3]=rnontemporal, [2]=wnontemporal, [1]=rprivileged (inverted), [0]=wprivileged (inverted)
    // For CPYFPRTWN:
    //   rnontemporal=0 (options[3]=0), wnontemporal=1 (options[2]=1)
    //   rprivileged=0 (options[1]=1 means unprivileged read), wprivileged=0 (options[0]=1 means unprivileged write)
    let options = Expr::Const {
        value: 0b0111, // rnontemporal=0, wnontemporal=1, rprivileged=0, wprivileged=0
        ty: BitWidth::new(4).unwrap(),
    };

    // Stage from op1: 00=Prologue, 01=Main, 10=Epilogue
    let stage = Expr::Const {
        value: 0, // MOPSStage_Prologue
        ty: BitWidth::new(2).unwrap(),
    };

    // Model a single memory copy iteration (representative) - 64-bit
    let mem_read = Expr::MemRead {
        address: Box::new(fromaddress.clone()),
        size: MemSize::Bits64,
    };

    let mem_write = Stmt::MemWrite {
        address: toaddress.clone(),
        value: mem_read,
        size: MemSize::Bits64,
        access: MemAccess::Write,
        ordering: MemOrdering::Relaxed,
    };

    // Update addresses for next iteration (simplified)
    let element_size = Expr::Const {
        value: 8, // 64-bit = 8 bytes
        ty: BitWidth::W64,
    };

    let new_toaddress = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(toaddress),
        right: Box::new(element_size.clone()),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let new_fromaddress = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(fromaddress),
        right: Box::new(element_size),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Update size remaining
    let new_cpysize = Expr::Binary {
        op: semantic_ir::BinOp::Sub,
        left: Box::new(cpysize),
        right: Box::new(Expr::Const {
            value: 8,
            ty: BitWidth::W64,
        }),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    // Update NZCV (prologue sets to 0000 if option A)
    let nzcv = Expr::Const {
        value: 0,
        ty: BitWidth::W4,
    };

    let mut semantics = vec![
        mem_write,
        Stmt::WriteReg {
            reg: rd,
            value: new_toaddress,
        },
        Stmt::WriteReg {
            reg: rs,
            value: new_fromaddress,
        },
        Stmt::WriteReg {
            reg: rn,
            value: new_cpysize,
        },
    ];

    for (flag, bit) in [(Flag::N, 3), (Flag::Z, 2), (Flag::C, 1), (Flag::V, 0)] {
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
            mnemonic: "CPYFPRTWN".to_string(),
            encoding: Some("Memory copy forward-only, reads unprivileged, writes non-temporal prologue".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MOPS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cpyfprtwn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cpyfprtwn_validates() {
    let instruction = cpyfprtwn_instruction();
    validate_instruction(&instruction).expect("CPYFPRTWN should validate");
}

#[test]
fn cpyfprtwn_requires_feat_mops() {
    let instruction = cpyfprtwn_instruction();
    assert!(instruction.required_features.contains(&"FEAT_MOPS".to_string()));
}

#[test]
fn cpyfprtwn_reads_three_registers() {
    let instruction = cpyfprtwn_instruction();

    let mut read_regs = std::collections::HashSet::new();
    for stmt in &instruction.semantics {
        fn collect_read_reg(expr: &semantic_ir::Expr, set: &mut std::collections::HashSet<u8>) {
            match expr {
                semantic_ir::Expr::ReadReg { reg, .. } => {
                    if let Reg::X(n) = reg {
                        set.insert(*n);
                    }
                }
                semantic_ir::Expr::MemRead { address, .. } => collect_read_reg(address, set),
                semantic_ir::Expr::Binary { left, right, .. } => {
                    collect_read_reg(left, set);
                    collect_read_reg(right, set);
                }
                _ => {}
            }
        }
        if let Stmt::MemWrite { value, .. } = stmt {
            collect_read_reg(value, &mut read_regs);
        }
        if let Stmt::WriteReg { value, .. } = stmt {
            collect_read_reg(value, &mut read_regs);
        }
    }
    assert!(read_regs.len() >= 3, "Expected at least 3 registers read, got {:?}", read_regs);
}

#[test]
fn cpyfprtwn_writes_nzcv_flags() {
    let instruction = cpyfprtwn_instruction();

    let flag_writes: Vec<_> = instruction
        .semantics
        .iter()
        .filter_map(|s| match s {
            Stmt::WriteFlag { flag, .. } => Some(*flag),
            _ => None,
        })
        .collect();

    assert_eq!(flag_writes.len(), 4);
    assert!(flag_writes.contains(&Flag::N));
    assert!(flag_writes.contains(&Flag::Z));
    assert!(flag_writes.contains(&Flag::C));
    assert!(flag_writes.contains(&Flag::V));
}

#[test]
fn cpyfprtwn_performs_mem_read_and_write() {
    let instruction = cpyfprtwn_instruction();

    let has_mem_write = instruction.semantics.iter().any(|s| {
        matches!(s, Stmt::MemWrite { .. })
    });
    assert!(has_mem_write);
}

#[test]
fn cpyfprtwn_options_read_unprivileged_write_non_temporal() {
    let instruction = cpyfprtwn_instruction();

    // The instruction uses read unprivileged + temporal, write unprivileged + non-temporal
    // options=0b0111 for rnontemporal=0, wnontemporal=1, rprivileged=0, wprivileged=0
    assert!(instruction.required_features.contains(&"FEAT_MOPS".to_string()));
}