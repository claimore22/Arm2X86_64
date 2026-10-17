/// ADDHN, ADDHN2 - Add returning high narrow
/// Stanford source: arm64_md_clean/addhn_advsimd.md
/// 
/// Stanford pseudocode:
///   let operand1 : bits(2*datasize) = V{}(n);  // 128-bit source
///   let operand2 : bits(2*datasize) = V{}(m);  // 128-bit source
///   var result : bits(datasize);               // 64-bit result
///   for e = 0 to elements-1 do
///       element1 = UInt(operand1[e*(2*esize)]);
///       element2 = UInt(operand2[e*(2*esize)]);
///       sum = element1 + element2;
///       sum = RShr(sum, esize, round);         // Shift right by esize (narrowing)
///       result[e*esize] = sum[esize-1:0];      // Take high half
///   end;
///   Vpart{datasize}(d, part) = result;         // Write to lower (ADDHN) or upper (ADDHN2) half
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn addhn_vector_instruction() -> Instruction {
    // ADDHN: 128-bit sources, 64-bit result (datasize=64), part=0 (lower half)
    // Example: esize=32 (4 bytes), elements = 64/32 = 2 elements
    let datasize = BitWidth::W64;    // Destination datasize
    let esize = BitWidth::W32;       // Source element size (32-bit), result element size = esize/2 = 16-bit? 
    // Wait: the operation is: add two 2*esize elements, shift right by esize, take esize bits
    // So source elements are 2*esize bits, result elements are esize bits
    // For esize=32: source elements are 64-bit, result elements are 32-bit
    // 2 elements of 32-bit result = 64-bit result

    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,  // Source is 128-bit (2*datasize)
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    // VectorNarrowing: takes 2*esize elements, adds, shifts right by esize, produces esize result elements
    let result = Expr::VectorNarrowing {
        op: semantic_ir::primitives::NarrowingOp::AddHN,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,  // Source datasize (2*datasize in pseudocode)
        esize: BitWidth::W64,      // Source element size (2*esize in pseudocode) = 64-bit
        round: false,              // ADDHN uses truncation (round=false)
        part: 0,                   // ADDHN writes to lower half (part=0)
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDHN".to_string(),
            encoding: Some("Add returning high narrow".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addhn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn addhn2_vector_instruction() -> Instruction {
    // ADDHN2: same as ADDHN but writes to upper half (part=1)
    let operand1 = Expr::ReadReg {
        reg: Reg::V(2),
        width: BitWidth::W128,
    };

    let operand2 = Expr::ReadReg {
        reg: Reg::V(3),
        width: BitWidth::W128,
    };

    let result = Expr::VectorNarrowing {
        op: semantic_ir::primitives::NarrowingOp::AddHN,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: BitWidth::W128,
        esize: BitWidth::W64,
        round: false,
        part: 1,  // ADDHN2 writes to upper half
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(1),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDHN2".to_string(),
            encoding: Some("Add returning high narrow (upper half)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addhn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addhn_vector_validates() {
    let instruction = addhn_vector_instruction();
    validate_instruction(&instruction).expect("ADDHN vector should validate");
}

#[test]
fn addhn2_vector_validates() {
    let instruction = addhn2_vector_instruction();
    validate_instruction(&instruction).expect("ADDHN2 vector should validate");
}

#[test]
fn addhn_vector_uses_vector_narrowing() {
    let instruction = addhn_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorNarrowing { op, left, right, datasize, esize, round, part } = value else {
        panic!("expected VectorNarrowing");
    };

    assert_eq!(*op, semantic_ir::primitives::NarrowingOp::AddHN);
    assert_eq!(*datasize, BitWidth::W128);  // Source is 128-bit
    assert_eq!(*esize, BitWidth::W64);      // Source element size is 64-bit
    assert_eq!(*round, false);              // Truncation
    assert_eq!(*part, 0);                   // Lower half
    assert!(matches!(left.as_ref(), Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }));
    assert!(matches!(right.as_ref(), Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }));
}

#[test]
fn addhn2_vector_writes_upper_half() {
    let instruction = addhn2_vector_instruction();
    
    let Stmt::WriteReg { value, .. } = &instruction.semantics[0] else {
        panic!("expected WriteReg");
    };

    let Expr::VectorNarrowing { part, .. } = value else {
        panic!("expected VectorNarrowing");
    };

    assert_eq!(*part, 1);  // Upper half
}

#[test]
fn addhn_vector_valid_datasize_esize_combinations() {
    // Test that validation accepts valid (datasize, esize) combinations
    // Stanford: datasize = 64, esize = 8, 16, or 32 (source elements 16, 32, 64-bit)
    // Our IR: source is 2*datasize = 128-bit, esize is the source element size
    
    // Valid: 128-bit source, 16-bit elements (esize=8)
    let valid1 = Instruction {
        id: InstructionId {
            mnemonic: "ADDHN".to_string(),
            encoding: Some("Add returning high narrow".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteReg {
            reg: Reg::V(1),
            value: Expr::VectorNarrowing {
                op: semantic_ir::primitives::NarrowingOp::AddHN,
                left: Box::new(Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }),
                right: Box::new(Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }),
                datasize: BitWidth::W128,
                esize: BitWidth::W16,  // 128 % 16 == 0, valid
                round: false,
                part: 0,
            },
        }],
        provenance: Provenance {
            source_path: "addhn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    };
    validate_instruction(&valid1).expect("esize=16 should be valid");

    // Valid: 128-bit source, 32-bit elements (esize=16)
    let valid2 = Instruction {
        id: InstructionId {
            mnemonic: "ADDHN".to_string(),
            encoding: Some("Add returning high narrow".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteReg {
            reg: Reg::V(1),
            value: Expr::VectorNarrowing {
                op: semantic_ir::primitives::NarrowingOp::AddHN,
                left: Box::new(Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }),
                right: Box::new(Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }),
                datasize: BitWidth::W128,
                esize: BitWidth::W32,  // 128 % 32 == 0, valid
                round: false,
                part: 0,
            },
        }],
        provenance: Provenance {
            source_path: "addhn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    };
    validate_instruction(&valid2).expect("esize=32 should be valid");

    // Valid: 128-bit source, 64-bit elements (esize=32)
    let valid3 = Instruction {
        id: InstructionId {
            mnemonic: "ADDHN".to_string(),
            encoding: Some("Add returning high narrow".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_AdvSIMD".to_string()],
        decode: None,
        semantics: vec![Stmt::WriteReg {
            reg: Reg::V(1),
            value: Expr::VectorNarrowing {
                op: semantic_ir::primitives::NarrowingOp::AddHN,
                left: Box::new(Expr::ReadReg { reg: Reg::V(2), width: BitWidth::W128 }),
                right: Box::new(Expr::ReadReg { reg: Reg::V(3), width: BitWidth::W128 }),
                datasize: BitWidth::W128,
                esize: BitWidth::W64,  // 128 % 64 == 0, valid
                round: false,
                part: 0,
            },
        }],
        provenance: Provenance {
            source_path: "addhn_advsimd.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    };
    validate_instruction(&valid3).expect("esize=64 should be valid");
}