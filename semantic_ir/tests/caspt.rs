/// CASPT, CASPAT, CASPALT, CASPLT semantics are representable in the IR.
/// Stanford source: arm64_md_clean/caspt.md
/// Stanford (arm64_md_no_nav_clean/caspt.md):
///   Decode:
///     if !IsFeatureImplemented(FEAT_LSUI) then EndOfDecode(Decode_UNDEF); end;
///     if Rs[0] == '1' || Rt[0] == '1' then EndOfDecode(Decode_UNDEF); end;
///     s = UInt(Rs); t = UInt(Rt); n = UInt(Rn);
///     datasize = 64; // Pair of 64-bit doublewords (128-bit memory operation)
///     acquire = L == '1';
///     release = o0 == '1';
///   Operation:
///     s1 = X[s, 64]; s2 = X[s+1, 64];
///     t1 = X[t, 64]; t2 = X[t+1, 64];
///     comparevalue = s2::s1; (little-endian)
///     newvalue = t2::t1;
///     if n == 31 then CheckSPAlignment(); address = SP[64];
///     else address = X[n, 64]; end;
///     data = MemAtomic(address, comparevalue, newvalue, accdesc);
///     X[s, 64] = data[63:0];
///     X[s+1, 64] = data[127:64];
///
/// Variants:
///   CASPT   : Relaxed ordering (L=0, o0=0)
///   CASPAT  : Acquire ordering (L=1, o0=0)
///   CASPLT  : Release ordering (L=0, o0=1)
///   CASPALT : AcquireRelease ordering (L=1, o0=1)
///
/// Uses `Expr::Concat` for pair comparison/store values, `Expr::MemAtomic` for
/// atomic compare-and-swap, and `Expr::Extract` to write back both destination registers.
/// Requires `FEAT_LSUI`.
use semantic_ir::{
    BitWidth, DecodeExpr, DecodeSpec, DecodeStmt, Expr, Instruction, InstructionId,
    MemOrdering, MemSize, Provenance, Reg, Stmt, validate_instruction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CasptVariant {
    Caspt,
    Caspat,
    Casplt,
    Caspalt,
}

impl CasptVariant {
    fn mnemonic(self) -> &'static str {
        match self {
            CasptVariant::Caspt => "CASPT",
            CasptVariant::Caspat => "CASPAT",
            CasptVariant::Casplt => "CASPLT",
            CasptVariant::Caspalt => "CASPALT",
        }
    }

    fn ordering(self) -> MemOrdering {
        match self {
            CasptVariant::Caspt => MemOrdering::Relaxed,
            CasptVariant::Caspat => MemOrdering::Acquire,
            CasptVariant::Casplt => MemOrdering::Release,
            CasptVariant::Caspalt => MemOrdering::AcquireRelease,
        }
    }
}

fn caspt_instruction(variant: CasptVariant, s: u8, t: u8, n: Option<u8>) -> Instruction {
    assert!(s % 2 == 0, "s must be even");
    assert!(t % 2 == 0, "t must be even");

    let reg_s = Reg::x(s).expect("valid x reg");
    let reg_s1 = Reg::x(s + 1).expect("valid x reg");
    let reg_t = Reg::x(t).expect("valid x reg");
    let reg_t1 = Reg::x(t + 1).expect("valid x reg");

    let s1 = Expr::ReadReg {
        reg: reg_s,
        width: BitWidth::W64,
    };
    let s2 = Expr::ReadReg {
        reg: reg_s1,
        width: BitWidth::W64,
    };
    let t1 = Expr::ReadReg {
        reg: reg_t,
        width: BitWidth::W64,
    };
    let t2 = Expr::ReadReg {
        reg: reg_t1,
        width: BitWidth::W64,
    };

    let comparevalue = Expr::Concat {
        high: Box::new(s2),
        low: Box::new(s1),
        width: BitWidth::W128,
    };

    let newvalue = Expr::Concat {
        high: Box::new(t2),
        low: Box::new(t1),
        width: BitWidth::W128,
    };

    let address = match n {
        Some(rn) => Expr::ReadReg {
            reg: Reg::x(rn).expect("valid base register"),
            width: BitWidth::W64,
        },
        None => Expr::ReadSP {
            width: BitWidth::W64,
        },
    };

    let atomic_op = Expr::MemAtomic {
        address: Box::new(address),
        expected: Box::new(comparevalue),
        new_value: Box::new(newvalue),
        size: MemSize::Bits128,
        ordering: variant.ordering(),
    };

    let semantics = vec![
        Stmt::WriteReg {
            reg: reg_s,
            value: Expr::Extract {
                value: Box::new(atomic_op.clone()),
                lsb: 0,
                width: BitWidth::W64,
            },
        },
        Stmt::WriteReg {
            reg: reg_s1,
            value: Expr::Extract {
                value: Box::new(atomic_op),
                lsb: 64,
                width: BitWidth::W64,
            },
        },
    ];

    Instruction {
        id: InstructionId {
            mnemonic: variant.mnemonic().to_string(),
            encoding: Some("Compare and swap pair unprivileged".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_LSUI".to_string()],
        decode: Some(DecodeSpec {
            statements: vec![
                DecodeStmt::Let {
                    name: "s".to_string(),
                    value: DecodeExpr::UInt("Rs".to_string()),
                },
                DecodeStmt::Let {
                    name: "t".to_string(),
                    value: DecodeExpr::UInt("Rt".to_string()),
                },
                DecodeStmt::Let {
                    name: "n".to_string(),
                    value: DecodeExpr::UInt("Rn".to_string()),
                },
            ],
        }),
        semantics,
        provenance: Provenance {
            source_path: "caspt.md".to_string(),
            source_sha256:
                "3e514a48cb3b23fb999be08823a6e593459996d1cdb96dfbfc2a8c12e069eca9".to_string(),
            clean_sha256:
                "5a6f79368d0e5819f6f0ea4e214a5b4d730ffca61869ee37e7f53eddca2ce3c0".to_string(),
        },
    }
}

#[test]
fn caspt_all_variants_validate() {
    let variants = [
        CasptVariant::Caspt,
        CasptVariant::Caspat,
        CasptVariant::Casplt,
        CasptVariant::Caspalt,
    ];

    for variant in variants {
        let ins = caspt_instruction(variant, 2, 4, Some(6));
        validate_instruction(&ins).expect("CASPT variant should validate");

        let ins_sp = caspt_instruction(variant, 2, 4, None);
        validate_instruction(&ins_sp).expect("CASPT with SP base should validate");
    }
}

#[test]
fn caspt_requires_feat_lsui() {
    let ins = caspt_instruction(CasptVariant::Caspt, 0, 2, Some(4));
    assert_eq!(ins.required_features, vec!["FEAT_LSUI".to_string()]);
}

#[test]
fn caspt_ordering_semantics() {
    assert_eq!(CasptVariant::Caspt.ordering(), MemOrdering::Relaxed);
    assert_eq!(CasptVariant::Caspat.ordering(), MemOrdering::Acquire);
    assert_eq!(CasptVariant::Casplt.ordering(), MemOrdering::Release);
    assert_eq!(CasptVariant::Caspalt.ordering(), MemOrdering::AcquireRelease);

    let ins = caspt_instruction(CasptVariant::Caspat, 2, 4, Some(6));
    let Stmt::WriteReg { value, .. } = &ins.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::Extract { value: inner, .. } = value else {
        panic!("expected Extract");
    };
    let Expr::MemAtomic { ordering, size, .. } = inner.as_ref() else {
        panic!("expected MemAtomic");
    };
    assert_eq!(*ordering, MemOrdering::Acquire);
    assert_eq!(*size, MemSize::Bits128);
}

#[test]
fn caspt_writes_both_64bit_destination_registers() {
    let ins = caspt_instruction(CasptVariant::Caspt, 2, 4, Some(6));
    assert_eq!(ins.semantics.len(), 2);

    let Stmt::WriteReg { reg: reg0, value: val0 } = &ins.semantics[0] else {
        panic!("expected WriteReg 0");
    };
    assert_eq!(*reg0, Reg::X(2));
    let Expr::Extract { lsb: lsb0, width: w0, .. } = val0 else {
        panic!("expected Extract 0");
    };
    assert_eq!(*lsb0, 0);
    assert_eq!(*w0, BitWidth::W64);

    let Stmt::WriteReg { reg: reg1, value: val1 } = &ins.semantics[1] else {
        panic!("expected WriteReg 1");
    };
    assert_eq!(*reg1, Reg::X(3));
    let Expr::Extract { lsb: lsb1, width: w1, .. } = val1 else {
        panic!("expected Extract 1");
    };
    assert_eq!(*lsb1, 64);
    assert_eq!(*w1, BitWidth::W64);
}

#[test]
fn caspt_decode_binds_registers() {
    let ins = caspt_instruction(CasptVariant::Caspalt, 8, 10, Some(12));
    let decode = ins.decode.expect("CASPT should have decode");
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "s"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "t"
    )));
    assert!(decode.statements.iter().any(|s| matches!(
        s,
        DecodeStmt::Let { name, .. } if name == "n"
    )));
}
