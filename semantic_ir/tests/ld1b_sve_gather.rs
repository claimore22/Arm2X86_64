/// LD1B (scalar plus vector) - Gather load unsigned bytes to vector (vector index)
/// Stanford source: arm64_md_clean/ld1b_z_p_bz.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P(g);
///   let base : bits(64) = if n == 31 then SP() else X(n);
///   let offset : bits(VL) = Z(VL)(m);
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(mask, e, esize) then
///           let offselt : bits(offs_size) = offset[e*esize][offs_size-1:0];
///           let off : integer = if offs_unsigned then UInt(offselt) else SInt(offselt);
///           let addr : bits(64) = AddressAdd(base, off << scale, accdesc);
///           data = Mem[msize](addr, accdesc);
///           result[e*esize] = Extend(esize)(data, unsigned);
///       else
///           result[e*esize] = Zeros{esize};
///       end;
///   end;
///   Z(VL)(t) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ld1b_sve_gather_instruction() -> Instruction {
    // Using VL=256, esize=64 (4 elements), msize=8 (byte load)
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;  // VL/8 = 256/8 = 32
    let esize = BitWidth::W64;  // Index elements are 64-bit
    let msize = BitWidth::W8;   // Loading bytes (8-bit)

    // Read predicate register P0
    let predicate = Expr::ReadPredReg {
        reg: 0,
        pl,
    };

    // Read base address from X0 (general-purpose register)
    let base = Expr::ReadReg {
        reg: semantic_ir::types::Reg::X(0),
        width: BitWidth::W64,
    };

    // Read vector index from Z1
    let index = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };

    // Read destination vector Z0 (for zeroing predication)
    let destination = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };

    // SVEGatherScatter for gather load
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(index),
        destination: Box::new(destination),
        vl,
        esize,
        msize,
        scale: 0,
        offs_unsigned: true,
        data_unsigned: true,
        first_fault: false,
        non_fault: false,
        non_temporal: false,
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1B".to_string(),
            encoding: Some("Gather load unsigned bytes to vector (vector index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1b_z_p_bz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1b_sve_gather_validates() {
    let instruction = ld1b_sve_gather_instruction();
    validate_instruction(&instruction).expect("LD1B SVE gather should validate");
}

#[test]
fn ld1b_sve_gather_uses_sve_gather_scatter() {
    let instruction = ld1b_sve_gather_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, .. } = value else {
        panic!("expected SVEGatherScatter for LD1B SVE");
    };

    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert!(matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 }));
    assert!(matches!(base.as_ref(), Expr::ReadReg { reg: semantic_ir::types::Reg::X(0), width: BitWidth::W64 }));
    assert!(matches!(index.as_ref(), Expr::ReadScalableVecReg { reg: 1, vl: BitWidth::W256 }));
    assert!(matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 }));
}

#[test]
fn ld1b_sve_gather_element_count() {
    let instruction = ld1b_sve_gather_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::SVEGatherScatter { vl, esize, .. } = value else {
        panic!("expected SVEGatherScatter");
    };

    // VL=256, esize=64 -> 4 elements
    let elements = vl.bits() / esize.bits();
    assert_eq!(elements, 4);
}