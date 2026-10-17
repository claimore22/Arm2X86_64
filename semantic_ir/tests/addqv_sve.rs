/// ADDQV - Add reduction of quadword vector segments
/// Stanford source: arm64_md_clean/addqv_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let segments : integer = VL DIV 128;
///   let elempersegment : integer = 128 DIV esize;
///   let mask : bits(PL) = P[g];
///   let operand : bits(VL) = if AnyActiveElement(mask, esize) then Z[n] else Zeros{VL};
///   var result : bits(128) = Zeros{};
///   var stmp : bits(128) = Zeros{};
///   var dtmp : integer;
///   for e = 0 to elempersegment-1 do
///       dtmp = 0;
///       for s = 0 to segments-1 do
///           if ActivePredicateElement(mask, s * elempersegment + e, esize) then
///               stmp = operand[s*:128];
///               dtmp = dtmp + UInt(stmp[e*:esize]);
///           end;
///       end;
///       result[e*:esize] = dtmp[esize-1:0];
///   end;
///   V[128](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn addqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // 32-bit elements
    let segments = vl.bits() / 128; // 2 segments
    let elempersegment = 128 / esize.bits(); // 4 elements per segment

    let g = 0u8; // Pg
    let n = 1u8; // Zn
    let d = 0u8; // Vd

    // Predicate register
    let predicate = Expr::ReadPredReg { reg: g, pl };

    // Source vector
    let operand = Expr::ReadScalableVecReg { reg: n, vl };

    // Use ScalableVectorReduce with AddQV to model the segment-wise reduction
    // The result is a 128-bit vector (V register) with elempersegment elements
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::AddQV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(d),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDQV".to_string(),
            encoding: Some("Add reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addqv_sve_validates() {
    let instruction = addqv_sve_instruction();
    validate_instruction(&instruction).expect("ADDQV SVE should validate");
}

#[test]
fn addqv_sve_uses_scalable_vector_reduce() {
    let instruction = addqv_sve_instruction();

    let Stmt::WriteReg { value, reg, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    assert_eq!(*reg, Reg::V(0));

    let Expr::ScalableVectorReduce { op, predicate, operand, vl, esize, initial_value: _ } = value else {
        panic!("expected ScalableVectorReduce for ADDQV SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEVectorReduceOp::AddQV);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W32);
    assert!(matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
    assert!(matches!(operand.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
}

#[test]
fn addqv_sve_segment_calculation() {
    let instruction = addqv_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::ScalableVectorReduce { vl, esize, .. } = value else {
        panic!("expected ScalableVectorReduce");
    };

    let segments = vl.bits() / 128; // 2 segments for VL=256
    let elempersegment = 128 / esize.bits(); // 4 elements per 128-bit segment

    assert_eq!(segments, 2);
    assert_eq!(elempersegment, 4);
    // Total result elements = elempersegment = 4
    // Result size = 128 bits = elempersegment * esize = 4 * 32
}