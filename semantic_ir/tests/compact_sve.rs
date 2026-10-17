/// COMPACT - Copy Active vector elements to lower-numbered elements
/// Stanford source: arm64_md_clean/compact_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[g];
///   let operand1 : bits(VL) = if AnyActiveElement(mask, esize) then Z[n] else Zeros{VL};
///   var result : bits(VL) = Zeros{};
///   var x : integer = 0;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(mask, e, esize) then
///           let element : bits(esize) = operand1[e*:esize];
///           result[x*:esize] = element;
///           x = x + 1;
///       end;
///   end;
///   Z[VL](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn compact_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let g = 0u8; // Pg
    let n = 1u8; // Zn
    let d = 0u8; // Zd

    // Predicate register
    let predicate = Expr::ReadPredReg { reg: g, pl };

    // Source vector
    let operand1 = Expr::ReadScalableVecReg { reg: n, vl };

    // Destination (zeroing predication since we start with zeros)
    let destination = Expr::ReadScalableVecReg { reg: d, vl };

    // Use SVESpecialOp::Compact to compact active elements
    // The operation takes predicate and source, produces compacted vector
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Compact,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand1)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "COMPACT".to_string(),
            encoding: Some("Copy Active vector elements to lower-numbered elements".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME2p2".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "compact_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn compact_sve_validates() {
    let instruction = compact_sve_instruction();
    validate_instruction(&instruction).expect("COMPACT SVE should validate");
}

#[test]
fn compact_sve_uses_special_op() {
    let instruction = compact_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for COMPACT");
    };

    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Compact);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert!(predicate.is_some());
    assert_eq!(operands.len(), 1);
    assert!(matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(predicate.as_ref().unwrap().as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
}