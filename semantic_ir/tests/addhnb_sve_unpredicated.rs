/// ADDHNB - Add narrow high part (bottom) (unpredicated)
/// Stanford source: arm64_md_clean/addhnb_z_zz.md
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Stmt,
    validate_instruction,
};

fn addhnb_sve_unpredicated_instruction() -> Instruction {
    let vl = BitWidth::W256;
    let esize = BitWidth::W64; // Source element size
    let halfesize = BitWidth::W32; // Result element size (half of esize)

    let operand1 = Expr::ReadScalableVecReg { reg: 1, vl };
    let operand2 = Expr::ReadScalableVecReg { reg: 2, vl };
    let destination = Expr::ReadScalableVecReg { reg: 0, vl };

    let result = Expr::VectorNarrowing {
        op: semantic_ir::primitives::NarrowingOp::AddHN,
        left: Box::new(operand1),
        right: Box::new(operand2),
        datasize: vl,
        esize, // Source element size
        round: false,
        part: 0, // Bottom part (ADDHNB)
    };

    let semantics = vec![Stmt::WriteScalableVecReg { reg: 0, value: result, vl }];

    Instruction {
        id: InstructionId {
            mnemonic: "ADDHNB".to_string(),
            encoding: Some("Add narrow high part (bottom) (unpredicated)".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE2".to_string(), "FEAT_SME".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "addhnb_z_zz.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn addhnb_sve_unpredicated_validates() {
    let instruction = addhnb_sve_unpredicated_instruction();
    validate_instruction(&instruction).expect("ADDHNB SVE unpredicated should validate");
}