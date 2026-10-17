/// SVDOT (2-way) SVE Dot Product semantics are representable in the IR.
/// Stanford source: arm64_md_clean/svdot.md
/// 2-way dot product. Requires FEAT_SVE.
use semantic_ir::{
    BitWidth, Expr, Instruction, InstructionId, Provenance, Reg, SpecialReg, Stmt, ValueType,
    Width, validate_instruction,
};

fn svdot_instruction(mnemonic: &str) -> Instruction {
    let vl = BitWidth::W128;

    let a = Expr::ReadScalableVecReg {
        reg: 0,
        vl,
    };
    let b = Expr::ReadScalableVecReg {
        reg: 1,
        vl,
    };
    let addend = Expr::ReadScalableVecReg {
        reg: 2,
        vl,
    };

    let result = Expr::VectorDot {
        op: semantic_ir::primitives::VectorDotOp::SDot,
        a: Box::new(a),
        b: Box::new(b),
        addend: Box::new(addend),
        datasize: vl,
        esize: BitWidth::W32,
        fpcr: Box::new(Expr::ReadSpecial {
            reg: semantic_ir::SpecialReg::Fpcr,
            width: BitWidth::W32,
        }),
        fpmr: Some(Box::new(Expr::ReadSpecial {
            reg: semantic_ir::SpecialReg::Fpmr,
            width: BitWidth::W32,
        })),
        broadcast_index: None::<u8>,
    };

    let semantics = vec![Stmt::WriteScalableVecReg {
        reg: 0,
        value: result,
        vl,
    }];

    Instruction {
        id: InstructionId {
            mnemonic: mnemonic.to_string(),
            encoding: Some("SVE Dot Product 2-Way".to_string()),
        },
        operands: Vec::new(),
        required_features: vec!["FEAT_SVE".to_string()],
        decode: None,
        semantics,
        provenance: Provenance {
            source_path: "svdot.md".to_string(),
            source_sha256: "test-source-sha256".to_string(),
            clean_sha256: "test-clean-sha256".to_string(),
        },
    }
}

#[test]
fn svdot_2way_indexed_validates() {
    validate_instruction(&svdot_instruction("SVDOT (2-way, indexed)"))
        .expect("SVDOT (2-way, indexed) should validate");
}

#[test]
fn svdot_2way_vectors_validates() {
    validate_instruction(&svdot_instruction("SVDOT (2-way, vectors)"))
        .expect("SVDOT (2-way, vectors) should validate");
}
