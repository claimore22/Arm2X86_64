/// ADDHNB, ADDHNT - Add narrow high part (bottom/top)
/// Stanford source: arm64_md_clean/addhnb_z_zz.md, addhnt_z_zz.md
///
/// Stanford pseudocode (ADDHNB):
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let operand1 : bits(VL) = Z[n];
///   let operand2 : bits(VL) = Z[m];
///   var result : bits(VL);
///   let halfesize : integer = esize DIV 2;
///   for e = 0 to elements-1 do
///       let element1 : integer = UInt(operand1[e*:esize]);
///       let element2 : integer = UInt(operand2[e*:esize]);
///       let res : integer = (element1 + element2) >> halfesize;
///       result[(2*e + 0)*:halfesize] = res[halfesize-1:0];
///       result[(2*e + 1)*:halfesize] = Zeros{halfesize};
///   end;
///   Z[VL](d) = result;
///
/// Stanford pseudocode (ADDHNT):
///   let VL : integer = CurrentVL();
///   let elements : integer = VL DIV esize;
///   let operand1 : bits(VL) = Z[n];
///   let operand2 : bits(VL) = Z[m];
///   var result : bits(VL) = Z[d];
///   let halfesize : integer = esize DIV 2;
///   for e = 0 to elements-1 do
///       let element1 : integer = UInt(operand1[e*:esize]);
///       let element2 : integer = UInt(operand2[e*:esize]);
///       let res : integer = (element1 + element2) >> halfesize;
///       result[(2*e + 1)*:halfesize] = res[halfesize-1:0];
///   end;
///   Z[VL](d) = result;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn create_addhn_instruction(mnemonic: &str, is_bottom: bool) -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // 64-bit source elements -> 32-bit result elements
    let halfesize = BitWidth::W32;
    let elements = vl.bits() / esize.bits(); // 4 elements

    let n = 1u8; // Zn
    let m = 2u8; // Zm
    let d = 0u8; // Zd

    let operand1 = Expr::ReadScalableVecReg { reg: n, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: m, vl };

    let destination = if !is_bottom {
        // ADDHNT: merge with existing destination
        Some(Expr::ReadScalableVecReg { reg: d, vl })
    } else {
        // ADDHNB: zero the odd elements, no merge
        None
    };

    // VectorNarrowing operation - takes wider elements, adds them, shifts right by halfesize
    // For ADDHNB: places result in even elements (2*e), zeros odd elements (2*e+1)
    // For ADDHNT: places result in odd elements (2*e+1), preserves even elements
    let result = Expr::VectorNarrowing {
        op: semantic_ir::primitives::NarrowingOp::AddHN,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: vl,           // Source datasize (256 bits)
        esize,                  // Source element size (64 bits)
        round: false,           // Truncate (not round)
        part: if is_bottom { 0 } else { 1 }, // 0 = lower half (even), 1 = upper half (odd)
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: d,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some(format!("Add narrow high part ({})", if is_bottom { "bottom" } else { "top" })),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: if is_bottom { "addhnb_z_zz.md" } else { "addhnt_z_zz.md" }.to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

// ADDHNB - Add narrow high part (bottom)
fn addhnb_sve_instruction() -> Instruction {
    create_addhn_instruction("ADDHNB", true)
}

// ADDHNT - Add narrow high part (top)
fn addhnt_sve_instruction() -> Instruction {
    create_addhn_instruction("ADDHNT", false)
}

#[test]
fn addhnb_sve_validates() {
    let instruction = addhnb_sve_instruction();
    validate_instruction(&instruction).expect("ADDHNB SVE should validate");
}

#[test]
fn addhnb_sve_uses_vector_narrowing() {
    let instruction = addhnb_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::VectorNarrowing { op, datasize, esize, round, part, .. } = value else {
        panic!("expected VectorNarrowing for ADDHNB");
    };

    assert_eq!(*op, semantic_ir::primitives::NarrowingOp::AddHN);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert!(*round == false);
    assert_eq!(*part, 0); // bottom = part 0
}

#[test]
fn addhnb_sve_destination_register() {
    let instruction = addhnb_sve_instruction();

    let Stmt::WriteScalableVecReg { reg, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    assert_eq!(*reg, 0u8);
}

#[test]
fn addhnt_sve_validates() {
    let instruction = addhnt_sve_instruction();
    validate_instruction(&instruction).expect("ADDHNT SVE should validate");
}

#[test]
fn addhnt_sve_uses_vector_narrowing() {
    let instruction = addhnt_sve_instruction();

    let Stmt::WriteScalableVecReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    let Expr::VectorNarrowing { op, datasize, esize, round, part, .. } = value else {
        panic!("expected VectorNarrowing for ADDHNT");
    };

    assert_eq!(*op, semantic_ir::primitives::NarrowingOp::AddHN);
    assert_eq!(*datasize, BitWidth::W256);
    assert_eq!(*esize, BitWidth::W64);
    assert!(*round == false);
    assert_eq!(*part, 1); // top = part 1
}

#[test]
fn addhnt_sve_destination_register() {
    let instruction = addhnt_sve_instruction();

    let Stmt::WriteScalableVecReg { reg, .. } = &instruction.semantics[0] else {
        panic!("expected WriteScalableVecReg");
    };

    assert_eq!(*reg, 0u8);
}