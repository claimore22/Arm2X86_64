/// LD1B (scalar plus scalar, single register) - Contiguous load unsigned bytes to vector (scalar index)
/// Stanford source: arm64_md_clean/ld1b_z_p_br.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   var base : bits(64);
///   let mask : bits(PL) = P[](g);
///   var result : bits(VL);
///   var data : bits(msize);
///   var offset : bits(64);
///   var addr : bits(64);
///   let mbytes : integer = msize DIV 8;
///   let contiguous : boolean = TRUE;
///   if !AnyActiveElement(PL)(mask, esize) then
///       base = if n == 31 then SP() else X[](n);
///       offset = X[](m);
///       addr = AddressAdd(base, UInt(offset) * mbytes, accdesc);
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
/// This is a predicated contiguous load with scalar base (X[n] or SP) and scalar index (X[m]).
/// The index is incremented after each element access but the index register is not updated.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn ld1b_sve_contiguous_scalar_index_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64; // Vector element size (64-bit)
    let msize = BitWidth::W8;  // Memory access size (8-bit bytes)

    let predicate = Expr::ReadPredReg { reg: 0, pl };

    // Base address: X[0] (general-purpose register)
    let base = Expr::ReadReg {
        reg: Reg::X(0),
        width: BitWidth::W64,
    };

    // Scalar index: X[1] (general-purpose register)
    let index = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // LD1B contiguous load with scalar base and scalar index
    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad,
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
            mnemonic: "LD1B".to_string(),
            encoding: Some("Contiguous load unsigned bytes to vector (scalar base, scalar index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1b_z_p_br.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1b_sve_contiguous_scalar_index_validates() {
    let instruction = ld1b_sve_contiguous_scalar_index_instruction();
    validate_instruction(&instruction).expect("LD1B contiguous scalar index should validate");
}

#[test]
fn ld1b_sve_contiguous_scalar_index_structure() {
    let instruction = ld1b_sve_contiguous_scalar_index_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, contiguous, .. } = value else {
        panic!("expected SVEGatherScatter for LD1B contiguous");
    };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::ContiguousLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W8);
    assert_eq!(*contiguous, true);
    matches!(predicate.as_ref(), Expr::ReadPredReg { reg: 0, pl: BitWidth::W32 });
    matches!(base.as_ref(), Expr::ReadReg { reg: Reg::X(0), width: BitWidth::W64 });
    matches!(index.as_ref(), Expr::ReadReg { reg: Reg::X(1), width: BitWidth::W64 });
    matches!(destination.as_ref(), Expr::ReadScalableVecReg { reg: 0, vl: BitWidth::W256 });
}