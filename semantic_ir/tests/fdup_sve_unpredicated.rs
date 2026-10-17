/// FDUP (SVE unpredicated) - Broadcast floating-point immediate to vector elements
/// Stanford source: arm64_md_clean/fdup_z_i.md
///
/// Stanford pseudocode:
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   var result : bits(VL);
///   for e = 0 to elements-1 do
///       result[e*esize] = imm;
///   end;
///   Z[](d) = result;
///
/// Unconditionally broadcasts a floating-point immediate (expanded from imm8 via VFPExpandImm)
/// into each element of the destination vector. Unpredicated.
/// Also known as FMOV (immediate, unpredicated).
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn fdup_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    // Floating-point immediate: FPOne (1.0) for esize=32
    let fp_imm = Expr::Const {
        value: 0x3f800000u128, // 1.0 in IEEE 754 single-precision
        ty: BitWidth::W32,
    };

    // Broadcast the FP immediate to all elements using SVESpecialOp::Dup
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Dup,
        predicate: None,
        operands: vec![Box::new(fp_imm)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl: BitWidth::W256,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FDUP".to_string(),
            encoding: Some("Broadcast floating-point immediate to vector elements (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fdup_z_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fdup_sve_unpredicated_validates() {
    let instruction = fdup_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("FDUP SVE unpredicated should validate");
}

#[test]
fn fdup_sve_unpredicated_structure() {
    let instruction = fdup_sve_unpredicated_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };
    let Expr::SVESpecial { op, operands, vl, esize, .. } = value else {
        panic!("expected SVESpecial for FDUP");
    };
    assert_eq!(*op, semantic_ir::primitives::SVESpecialOp::Dup);
    assert_eq!(*vl, BitWidth::W256);
    assert_eq!(*esize, Some(BitWidth::W32));
    assert_eq!(operands.len(), 1);
    matches!(operands[0].as_ref(), Expr::Const { value: 0x3f800000, ty: BitWidth::W32 });
}