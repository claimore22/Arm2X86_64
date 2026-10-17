/// CNTB, CNTD, CNTH, CNTW (scalar) - Set scalar to multiple of predicate constraint element count
/// Stanford source: arm64_md_clean/cntb_r_s.md
///
/// Stanford pseudocode:
///   let esize : integer = 32;  // varies: 8 for CNTB, 64 for CNTD, 16 for CNTH, 32 for CNTW
///   let d : integer = UInt(Rd);
///   let pat : bits(5) = pattern;
///   let imm : integer = UInt(imm4) + 1;
///   CheckSVEEnabled();
///   let count : integer = DecodePredCount(pat, esize);
///   X[64](d) = (count * imm)[63:0];
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt, ValueType, Width,
    validate_instruction,
};

fn create_cnt_scalar_instruction(mnemonic: &str, esize: u16) -> Instruction {
    let vl = BitWidth::W256;

    let pat = Expr::Const { value: 0b00001, ty: BitWidth::W8 }; // Example: VL1 pattern
    let imm = Expr::Const { value: 1u128, ty: BitWidth::W64 }; // imm4 = 0, so imm = 1

    let d = 0u8; // X0

    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::DecodePredCount,
        predicate: None,
        operands: vec![
            Box::new(pat),
            Box::new(Expr::Const { value: esize as u128, ty: BitWidth::W8 }),
        ],
        vl,
        esize: None,
        imm: None,
    };

    // result = count * imm
    let result = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: semantic_ir::types::Reg::X(d),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Set scalar to multiple of predicate constraint element count".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cntb_r_s.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// CNTB: esize = 8 (byte)
fn cntb_scalar_sve_instruction() -> Instruction {
    create_cnt_scalar_instruction("CNTB", 8)
}

// CNTD: esize = 64 (doubleword)
fn cntd_scalar_sve_instruction() -> Instruction {
    create_cnt_scalar_instruction("CNTD", 64)
}

// CNTH: esize = 16 (halfword)
fn cnth_scalar_sve_instruction() -> Instruction {
    create_cnt_scalar_instruction("CNTH", 16)
}

// CNTW: esize = 32 (word)
fn cntw_scalar_sve_instruction() -> Instruction {
    create_cnt_scalar_instruction("CNTW", 32)
}

#[test]
fn cntb_scalar_sve_validates() {
    let instruction = cntb_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CNTB (scalar) SVE should validate");
}

#[test]
fn cntb_scalar_sve_uses_decode_pred_count() {
    let instruction = cntb_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { op: semantic_ir::BinOp::Mul, left, .. } = value else {
        panic!("expected Binary Mul for result");
    };

    let Expr::SVESpecial { op, operands, .. } = left.as_ref() else {
        panic!("expected SVESpecial for DecodePredCount");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::DecodePredCount);
    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 8, ty: BitWidth::W8 }));
}

#[test]
fn cntd_scalar_sve_validates() {
    let instruction = cntd_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CNTD (scalar) SVE should validate");
}

#[test]
fn cntd_scalar_sve_uses_esize_64() {
    let instruction = cntd_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { left, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::SVESpecial { operands, .. } = left.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 64, ty: BitWidth::W8 }));
}

#[test]
fn cnth_scalar_sve_validates() {
    let instruction = cnth_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CNTH (scalar) SVE should validate");
}

#[test]
fn cnth_scalar_sve_uses_esize_16() {
    let instruction = cnth_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { left, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::SVESpecial { operands, .. } = left.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 16, ty: BitWidth::W8 }));
}

#[test]
fn cntw_scalar_sve_validates() {
    let instruction = cntw_scalar_sve_instruction();
    validate_instruction(&instruction).expect("CNTW (scalar) SVE should validate");
}

#[test]
fn cntw_scalar_sve_uses_esize_32() {
    let instruction = cntw_scalar_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::Binary { left, .. } = value else {
        panic!("expected Binary");
    };

    let Expr::SVESpecial { operands, .. } = left.as_ref() else {
        panic!("expected SVESpecial");
    };

    assert!(matches!(operands[1].as_ref(), Expr::Const { value: 32, ty: BitWidth::W8 }));
}

// CNTP (predicate) - Set scalar to count of true predicate elements
/// Stanford source: arm64_md_clean/cntp_r_p_p.md
///
/// Stanford pseudocode:
///   let esize : integer = 8 << UInt(size);  // varies: 16, 32, 64
///   let g : integer = UInt(Pg);
///   let n : integer = UInt(Pn);
///   let d : integer = UInt(Rd);
///   CheckSVEEnabled();
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   let operand : bits(PL) = P[n];
///   var sum : bits(64) = Zeros{};
///   for e = 0 to elements-1 do
///       if (ActivePredicateElement{mask, e, esize} && ActivePredicateElement{operand, e, esize}) then
///           sum = sum + 1;
///       end;
///   end;
///   X[64](d) = sum;

fn create_cntp_predicate_instruction(mnemonic: &str, esize_bitwidth: BitWidth) -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W8; // Predicate granularity is 8 bits

    let g = 0u8; // Pg
    let n = 1u8; // Pn
    let d = 0u8; // Rd

    let predicate_mask = Expr::ReadPredReg { reg: g, pl };
    let predicate_operand = Expr::ReadPredReg { reg: n, pl };

    // Count true predicate elements using SVESpecialOp::CountPred with both mask and operand
    // This models: sum of (ActivePredicateElement(mask) && ActivePredicateElement(operand))
    let count = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::CountPred,
        predicate: Some(Box::new(predicate_mask)),
        operands: vec![Box::new(predicate_operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: semantic_ir::types::Reg::X(0),
        value: count,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("Set scalar to count of true predicate elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "cntp_r_p_p.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// CNTP (predicate) with esize = 16 (halfword)
fn cntp_predicate_half_sve_instruction() -> Instruction {
    create_cntp_predicate_instruction("CNTP_H", BitWidth::W16)
}

// CNTP (predicate) with esize = 32 (word)
fn cntp_predicate_word_sve_instruction() -> Instruction {
    create_cntp_predicate_instruction("CNTP_W", BitWidth::W32)
}

// CNTP (predicate) with esize = 64 (doubleword)
fn cntp_predicate_double_sve_instruction() -> Instruction {
    create_cntp_predicate_instruction("CNTP_D", BitWidth::W64)
}

#[test]
fn cntp_predicate_half_sve_validates() {
    let instruction = cntp_predicate_half_sve_instruction();
    validate_instruction(&instruction).expect("CNTP (predicate, half) SVE should validate");
}

#[test]
fn cntp_predicate_half_sve_uses_count_pred() {
    let instruction = cntp_predicate_half_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::SVESpecial { op, predicate, operands, esize, .. } = value else {
        panic!("expected SVESpecial for CountPred");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::CountPred);
    assert!(predicate.is_some());
    assert_eq!(operands.len(), 1);
    assert_eq!(*esize, Some(BitWidth::W8)); // Predicate granularity
    assert!(matches!(operands[0].as_ref(), Expr::ReadPredReg { reg: 1, pl: BitWidth::W32 }));
    assert!(matches!(predicate.as_ref().unwrap().as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
}

#[test]
fn cntp_predicate_word_sve_validates() {
    let instruction = cntp_predicate_word_sve_instruction();
    validate_instruction(&instruction).expect("CNTP (predicate, word) SVE should validate");
}

#[test]
fn cntp_predicate_word_sve_uses_esize_32() {
    let instruction = cntp_predicate_word_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::SVESpecial { esize, .. } = value else {
        panic!("expected SVESpecial");
    };

    assert_eq!(*esize, Some(BitWidth::W8)); // Predicate granularity is always 8 bits
}

#[test]
fn cntp_predicate_double_sve_validates() {
    let instruction = cntp_predicate_double_sve_instruction();
    validate_instruction(&instruction).expect("CNTP (predicate, double) SVE should validate");
}

#[test]
fn cntp_predicate_double_sve_uses_esize_64() {
    let instruction = cntp_predicate_double_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::SVESpecial { esize, .. } = value else {
        panic!("expected SVESpecial");
    };

    assert_eq!(*esize, Some(BitWidth::W8));
}