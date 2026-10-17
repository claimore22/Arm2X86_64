/// LD1D (vector plus immediate) - Gather load doublewords to vector (immediate index)
/// Stanford source: arm64_md_clean/ld1d_z_p_ai.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let PL : integer = VL DIV 8;
///   let elements : integer = VL DIV esize;
///   let mask : bits(PL) = P[](g);
///   var base : bits(VL);
///   var result : bits(VL);
///   if AnyActiveElement(PL)(mask, esize) then base = Z[](n); end;
///   for e = 0 to elements-1 do
///       if ActivePredicateElement(PL)(mask, e, esize) then
///           let baddr : bits(64) = ZeroExtend(base[e*esize]);
///           let addr : bits(64) = AddressAdd(baddr, offset * mbytes, accdesc);
///           data = Mem[msize](addr, accdesc);
///           result[e*esize] = Extend(esize)(data, unsigned);
///       else
///           result[e*esize] = Zeros(esize);
///       end;
///   end;
///   Z[](t) = result;
///
/// Gather load with vector base (Zn) and immediate index (imm5, multiple of 8, range 0-248).
/// msize = 64 (doublewords), mbytes = 8.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn ld1d_sve_gather_imm_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W64;
    let msize = BitWidth::W64; // Doublewords (64-bit)

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let base = Expr::ReadScalableVecReg { reg: 1, vl }; // Vector base Zn
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    // Immediate index: imm5 = 4 (multiple of 8, range 0-248)
    let imm_index = Expr::Const { value: 4, ty: BitWidth::new(5).unwrap() }; // 5-bit immediate

    let result = Expr::SVEGatherScatter {
        op: semantic_ir::primitives::SVEGatherScatterOp::GatherLoad,
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
        contiguous: false,
        tag_checked: true,
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "LD1D".to_string(),
            encoding: Some("Gather load doublewords to vector (immediate index)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "ld1d_z_p_ai.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn ld1d_sve_gather_imm_validates() {
    let instruction = ld1d_sve_gather_imm_instruction();
    validate_instruction(&instruction).expect("LD1D SVE gather immediate should validate");
}

#[test]
fn ld1d_sve_gather_imm_uses_sve_gather_scatter() {
    let instruction = ld1d_sve_gather_imm_instruction();
    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else { panic!("expected WriteScalableVecReg"); };
    let Expr::SVEGatherScatter { op, predicate, base, index, destination, vl, esize, msize, .. } = value else { panic!("expected SVEGatherScatter for LD1D immediate"); };
    assert_eq!(*op, semantic_ir::primitives::SVEGatherScatterOp::GatherLoad);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert_eq!(*msize, BitWidth::W64);
    assert_eq!(*index, Box::new(Expr::Const { value: 4, ty: BitWidth::new(5).unwrap() }));
}