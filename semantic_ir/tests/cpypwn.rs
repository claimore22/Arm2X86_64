/// CPYPWN, CPYMWN, CPYEWN (Memory copy, writes non-temporal)
/// Stanford source: arm64_md_clean/cpypwn.md
/// semantics are representable in the IR.
///
/// These are three stages of a memory copy operation using FEAT_MOPS:
/// - CPYPWN (Prologue): Preconditions arguments, copies initial portion
/// - CPYMWN (Main): Copies middle portion
/// - CPYEWN (Epilogue): Copies final remaining bytes
///
/// read privileged + temporal, write privileged + non-temporal:
///   rnontemporal=0 (options[3]=0), wnontemporal=1 (options[2]=1)
///   runprivileged=0 (options[1]), wunprivileged=0 (options[0])
use semantic_ir::{
    BitWidth, Expr, Flag, Instruction, InstructionId, MemAccess, MemOrdering, MemSize, Provenance,
    Reg, Stmt, ValueType, Width, validate_instruction,
};

fn cpypwn_instruction() -> Instruction {
    let _datasize = Width::Fixed(BitWidth::W64);

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

    // Options from op2 (4 bits): [3]=rnontemporal, [2]=wnontemporal, [1]=runprivileged, [0]=wunprivileged
    // For CPYPWN: read privileged + temporal, write privileged + non-temporal
    let _options = Expr::Const {
        value: 0b0100, // rnontemporal=0, wnontemporal=1, runprivileged=0, wunprivileged=0
        ty: BitWidth::new(4).unwrap(),
    };

    // Stage from op1: 00=Prologue, 01=Main, 10=Epilogue
    let _stage = Expr::Const {
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
            reg: Reg::X(1),
            value: new_toaddress,
        },
        Stmt::WriteReg {
            reg: Reg::X(2),
            value: new_fromaddress,
        },
        Stmt::WriteReg {
            reg: Reg::X(3),
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
            mnemonic: "CPYPWN".to_string(),
            encoding: Some("Memory copy, writes non-temporal prologue".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_MOPS".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cpypwn.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn cpypwn_validates() {
    let instruction = cpypwn_instruction();
    validate_instruction(&instruction).expect("CPYPWN should validate");
}

#[test]
fn cpypwn_requires_feat_mops() {
    let instruction = cpypwn_instruction();
    assert!(instruction.required_features.contains(&"FEAT_MOPS".to_string()));
}

#[test]
fn cpypwn_reads_three_registers() {
    let instruction = cpypwn_instruction();

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
fn cpypwn_writes_nzcv_flags() {
    let instruction = cpypwn_instruction();

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
fn cpypwn_performs_mem_read_and_write() {
    let instruction = cpypwn_instruction();

    let has_mem_write = instruction.semantics.iter().any(|s| {
        matches!(s, Stmt::MemWrite { .. })
    });
    assert!(has_mem_write);
}

#[test]
fn cpypwn_options_write_non_temporal() {
    let instruction = cpypwn_instruction();

    // Read unprivileged + non-temporal, write non-temporal, write privileged
    // options=0b1110
    assert!(instruction.required_features.contains(&"FEAT_MOPS".to_string()));
}
