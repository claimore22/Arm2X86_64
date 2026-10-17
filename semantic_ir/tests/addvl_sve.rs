/// ADDVL - Add multiple of vector register size to scalar register
/// Stanford source: arm64_md_clean/addvl_r_ri.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn addvl_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;

    let operand1 = Expr::ReadReg {
        reg: Reg::X(1),
        width: BitWidth::W64,
    };

    let result = Expr::Binary {
        op: semantic_ir::expr::BinOp::Add,
        left: Box::new(operand1),
        right: Box::new(Expr::Const { value: 4 * 32, ty: BitWidth::W64 }), // imm=4, VL/8=256/8=32
        ty: semantic_ir::types::ValueType::Bits(semantic_ir::types::Width::Fixed(BitWidth::W64)),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDVL".to_string(),
            encoding: Some("Add multiple of vector register size to scalar register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addvl_r_ri.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addvl_sve_validates() {
    let instruction = addvl_sve_instruction();
    validate_instruction(&instruction).expect("ADDVL SVE should validate");
}