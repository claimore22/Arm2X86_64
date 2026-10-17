/// INCW (scalar) - Increment scalar by multiple of predicate constraint element count (word)
/// Stanford source: arm64_md_clean/incb_r_rs.md
extern crate semantic_ir;
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt, ValueType, Width,
    validate_instruction,
};

fn incw_instruction() -> Instruction {
    // esize = 32 from the decode section
    let _esize = BitWidth::W32;
    
    // For the predicate pattern, we'll use a placeholder constant
    // In a real implementation, this would come from the instruction field
    let _pat = Expr::Const {
        value: 0, // Placeholder for 5-bit pattern
        ty: BitWidth::new(5).unwrap(),
    };
    
    // imm = UInt(imm4) + 1, so range 1-16
    // Using a placeholder value
    let imm_val = Expr::Const {
        value: 1, // Placeholder for imm4 value (0-15), so +1 gives 1-16
        ty: BitWidth::W4,
    };
    
    // We need to compute: count = DecodePredCount(pat, esize)
    // Since we don't have a primitive for DecodePredCount, we'll represent it
    // as a constant for now, with the understanding that it should be replaced
    // with the proper decoding logic
    let count = Expr::Const {
        value: 1, // Placeholder - in reality this depends on pat and esize
        ty: BitWidth::W32, // Assuming count fits in 32 bits
    };
    
    // Compute: count * imm
    // First, we need to zero-extend the immediate to match count width
    let imm_extended = Expr::ZeroExtend {
        value: Box::new(imm_val),
        to: BitWidth::W32,
    };
    
    let count_times_imm = Expr::Binary {
        op: semantic_ir::BinOp::Mul,
        left: Box::new(count),
        right: Box::new(imm_extended),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W32)),
    };
    
    // Extend count_times_imm to W64 for addition with operand1 (W64)
    let count_times_imm_extended = Expr::ZeroExtend {
        value: Box::new(count_times_imm),
        to: BitWidth::W64,
    };
    
    // Read the source register (X[dn])
    // The register number itself is just a small integer, we'll use W5 to represent it can hold values 0-31
    let _dn = Expr::Const {
        value: 0, // Placeholder for register number
        ty: BitWidth::new(5).unwrap(), // 5 bits for register specifier (0-31)
    };
    
    let operand1 = Expr::ReadReg {
        reg: Reg::X(0), // Placeholder register
        width: BitWidth::W64, // Scalar registers are 64-bit
    };
    
    // Compute: operand1 + (count * imm)
    // Use the extended count_times_imm for addition with operand1 (W64)
    let add_result = Expr::Binary {
        op: semantic_ir::BinOp::Add,
        left: Box::new(operand1),
        right: Box::new(count_times_imm_extended),
        ty: ValueType::Bits(Width::Fixed(BitWidth::W64)),
    };
    
    // Write the result back to X[dn]
    let write_reg = Stmt::WriteReg {
        reg: Reg::X(0), // Placeholder register
        value: add_result,
    };
    
    Instruction {
        id: InstructionId {
            mnemonic: "INCW".to_string(),
            encoding: Some("Increment scalar by multiple of predicate constraint element count (word)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics: vec![write_reg],
        provenance: Provenance {
            source_path: "incb_r_rs.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn incw_validates() {
    let instruction = incw_instruction();
    validate_instruction(&instruction).expect("INCW should validate");
}