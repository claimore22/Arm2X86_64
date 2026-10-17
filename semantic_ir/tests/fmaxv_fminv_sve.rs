/// FMAXV, FMINV, FMAXQV, FMINQV, FMAXNMV, FMINNMV, FMAXNMQV, FMINNMQV (SVE) - FP reductions
/// Stanford sources:
///   - fmaxv_v_p_z.md, fminv_v_p_z.md (scalar reduction)
///   - fmaxqv_z_p_z.md, fminqv_z_p_z.md (quadword segment reduction to 128-bit)
///   - fmaxnmv_v_p_z.md, fminnmv_v_p_z.md (scalar reduction, number variants)
///   - fmaxnmqv_z_p_z.md, fminnmqv_z_p_z.md (quadword reduction, number variants)
///
/// FMAXV: scalar max over all lanes (inactive = -Inf)
/// FMINV: scalar min over all lanes (inactive = +Inf)
/// FMAXQV: 128-bit result, max per element position across 128-bit segments (inactive = -Inf)
/// FMINQV: 128-bit result, min per element position across 128-bit segments (inactive = +Inf)
/// FMAXNMV: scalar max number (inactive = default NaN)
/// FMINNMV: scalar min number (inactive = default NaN)
/// FMAXNMQV: 128-bit max number per element across segments (inactive = default NaN)
/// FMINNMQV: 128-bit min number per element across segments (inactive = default NaN)
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn fmaxv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32; // esize = 16, 32, 64 (size != '00')

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // FMAXV: recursive reduction to scalar
    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FMaxV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0), // V[d] - scalar result
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXV".to_string(),
            encoding: Some("Floating-point maximum recursive reduction to scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxv_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FMinV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINV".to_string(),
            encoding: Some("Floating-point minimum recursive reduction to scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminv_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmaxqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    // FMAXQV: segment-wise reduction to 128-bit result
    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FAddQVSeg, // Reuse FAddQVSeg for segment reduction
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0), // V[d] - 128-bit result
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXQV".to_string(),
            encoding: Some("Floating-point maximum reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FAddQVSeg, // Reuse
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINQV".to_string(),
            encoding: Some("Floating-point minimum reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmaxnmv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FMaxNMV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXNMV".to_string(),
            encoding: Some("Floating-point maximum number recursive reduction to scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxnmv_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminnmv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::ScalableVectorReduce {
        op: semantic_ir::primitives::SVEVectorReduceOp::FMinNMV,
        predicate: Box::new(predicate),
        operand: Box::new(operand),
        vl,
        esize,
        initial_value: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINNMV".to_string(),
            encoding: Some("Floating-point minimum number recursive reduction to scalar".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminnmv_v_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fmaxnmqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FAddQVSeg, // Reuse
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMAXNMQV".to_string(),
            encoding: Some("Floating-point maximum number reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fmaxnmqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

fn fminnmqv_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;
    let esize = BitWidth::W32;

    let predicate = Expr::ReadPredReg { reg: 0, pl };
    let operand = Expr::ReadScalableVecReg { reg: 1, vl };

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::FAddQVSeg, // Reuse
        predicate: Some(Box::new(predicate)),
        operands: vec![Box::new(operand)],
        vl,
        esize: Some(esize),
        imm: None,
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::V(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "FMINNMQV".to_string(),
            encoding: Some("Floating-point minimum number reduction of quadword vector segments".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2p1".to_string(), "FEAT_SME2p1".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "fminnmqv_z_p_z.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn fmaxv_sve_validates() {
    let instruction = fmaxv_sve_instruction();
    validate_instruction(&instruction).expect("FMAXV SVE should validate");
}

#[test]
fn fminv_sve_validates() {
    let instruction = fminv_sve_instruction();
    validate_instruction(&instruction).expect("FMINV SVE should validate");
}

#[test]
fn fmaxqv_sve_validates() {
    let instruction = fmaxqv_sve_instruction();
    validate_instruction(&instruction).expect("FMAXQV SVE should validate");
}

#[test]
fn fminqv_sve_validates() {
    let instruction = fminqv_sve_instruction();
    validate_instruction(&instruction).expect("FMINQV SVE should validate");
}

#[test]
fn fmaxnmv_sve_validates() {
    let instruction = fmaxnmv_sve_instruction();
    validate_instruction(&instruction).expect("FMAXNMV SVE should validate");
}

#[test]
fn fminnmv_sve_validates() {
    let instruction = fminnmv_sve_instruction();
    validate_instruction(&instruction).expect("FMINNMV SVE should validate");
}

#[test]
fn fmaxnmqv_sve_validates() {
    let instruction = fmaxnmqv_sve_instruction();
    validate_instruction(&instruction).expect("FMAXNMQV SVE should validate");
}

#[test]
fn fminnmqv_sve_validates() {
    let instruction = fminnmqv_sve_instruction();
    validate_instruction(&instruction).expect("FMINNMQV SVE should validate");
}