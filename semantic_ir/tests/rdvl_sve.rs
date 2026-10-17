/// RDVL - Read multiple of vector register size to scalar register
/// Stanford source: arm64_md_clean/rdvl_r_i.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, Stmt,
    validate_instruction,
};

fn rdvl_sve_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let pl = BitWidth::W32;

    let result = Expr::SVESpecial {
        op: semantic_ir::primitives::SVESpecialOp::Rdvl,
        predicate: None,
        operands: vec![Box::new(Expr::Const { value: 4, ty: BitWidth::W8 })], // imm6 = 4
        vl,
        esize: None,
        imm: Some(4),
    };

    let semantics = vec![Stmt::WriteReg {
        reg: Reg::X(0),
        value: result,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: "RDVL".to_string(),
            encoding: Some("Read multiple of vector register size to scalar register".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "rdvl_r_i.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn rdvl_sve_validates() {
    let instruction = rdvl_sve_instruction();
    validate_instruction(&instruction).expect("RDVL SVE should validate");
}