/// FADDQV (SVE) - Floating-point add recursive reduction of quadword vector segments
/// Stanford source: arm64_md_clean/faddqv_z_p_z.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let segments : integer = VL DIV 128;
///   let elempersegment : integer = 128 DIV esize;
///   let segbits : integer = segments * esize;
///   let mask : bits(PL) = P[](g);
///   let operand : bits(VL) = if AnyActiveElement(PL)(mask, esize) then Z[](n) else Zeros(VL);
///   let identity : bits(esize) = FPZero(esize)('0');
///   var result : bits(128) = Zeros;
///   for e = 0 to elempersegment-1 do
///       var stmp : bits(segbits);
///       for s = 0 to segments-1 do
///           if ActivePredicateElement(PL)(mask, s * elempersegment + e, esize) then
///               stmp[s*esize] = operand[(s * elempersegment + e)*esize];
///           else
///               stmp[s*esize] = identity;
///           end;
///       end;
///       result[e*esize] = FPReduce(esize, segbits)(ReduceOp_FADD, stmp, FPCR());
///   end;
///   V[128](d) = result;
///
/// This reduces each element position across 128-bit vector segments.
/// For example, with VL=256 and esize=32:
///   - 2 segments of 128 bits each
///   - 4 elements per segment (128/32)
///   - Result is 128 bits (4 elements), each element is the sum of corresponding elements from both segments
/// Result goes to V[d] (128-bit SIMD&FP register)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn faddqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl }; // Z[n]

    // FADDQV reduces across 128-bit segments, result is 128 bits
    // Using SVESpecialOp::FAddQVSeg with the source vector and predicate
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FAddQVSeg,
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0), // V[d] (128-bit result)
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FADDQV".to_string(),
            encoding: Some("Floating-point add recursive reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "faddqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn faddqv_sve_validates() {
    let instruction = faddqv_sve_instruction();
    validate_instruction(&instruction).expect("FADDQV SVE should validate");
}

#[test]
fn faddqv_sve_structure() {
    let instruction = faddqv_sve_instruction();

    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };
    let Expr::SVESpecial { op, predicate, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for FADDQV");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::FAddQVSeg);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert!(predicate.is_some());
    matches!(predicate.as_ref().unwrap().as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 });
}