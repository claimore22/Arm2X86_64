/// LD1H (scalar plus immediate, single register) - Contiguous load unsigned halfwords to vector (immediate index)
/// Stanford source: arm64_md_clean/ld1h_z_p_bi.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   var base : bits(64);
///   let mask : bits(PL) = P[](g);
///   var addr : bits(64);
///   var result : bits(VL);
///   var data : bits(msize);
///   let mbytes : integer = msize DIV 8;
///   let contiguous : boolean = TRUE;
///   if !AnyActiveElement(PL)(mask, esize) then
///       base = if n == 31 then SP() else X[](n);
///       addr = AddressAdd(base, offset * elements * mbytes, accdesc);
///   end;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           data = Mem[msize](addr, accdesc);
///           result[e*esize] = Extend(esize)(data, unsigned);
///       else
///           result[e*esize] = Zeros{esize};
///       end;
///       addr = AddressIncrement(addr, mbytes, accdesc);
///   end;
///   Z[](t) = result;
///
/// This is a predicated contiguous load with scalar base (X[n] or SP) and immediate index.
/// The immediate index is in range -8 to 7 (SInt(imm4)), multiplied by elements * mbytes.
/// For halfwords, msize = 16, mbytes = 2.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1h_sve_contiguous_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector element size (64-bit)
    let msize = BitWidth::W16; // Memory access size (16-bit halfwords)

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    // Base address: X[0] (general-purpose register)
    let base = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    // Immediate index: imm4 = 3 (example, range -8 to 7)
    let imm_index = Expr::Const {
        value: 3i32 as u128,
        ty: BitWidth::W4,
    };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // LD1H contiguous load with scalar base and immediate index
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
        predicate: Box::new(predicate),
        base: Box::new(base),
        index: Box::new(imm_index),
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
        contiguous: true,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1H".to_string(),
            encoding: Some("Contiguous load unsigned halfwords to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1h_z_p_bi.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1h_sve_contiguous_imm_validates() {
    let instruction = ld1h_sve_contiguous_imm_instruction();
    validate_instruction(&instruction).expect("LD1H contiguous immediate should validate");
}

#[test]
fn ld1h_sve_contiguous_imm_structure() {
    let instruction = ld1h_sve_contiguous_imm_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
        panic!("expected SVEGatherScatter for LD1H contiguous immediate");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W16);
    assert_eq!(*contiguous, true);
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::Const { value: 3, ty: BitWidth::W4 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}