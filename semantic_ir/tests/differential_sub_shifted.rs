mod common;

use common::{percentile, qemu_available, run_differential_suite_op, ArithmeticOp};

#[test]
fn differential_sub_shifted_suite() {
    let cases: [(u64, u64); 6] = [
        (0, 0),
        (5, 3),
        (0, 1),
        (10, 20),
        (u64::MAX, 1),
        (u64::MIN, u64::MAX),
    ];
    let mismatches = run_differential_suite_op(ArithmeticOp::Sub, &cases)
        .expect("differential suite should run");
    assert!(
        mismatches.is_empty(),
        "differential mismatches: {:?}",
        mismatches
    );

    let out = common::run_differential_case_op(common::ArithmeticOp::Sub, cases[0].0, cases[0].1)
        .expect("qemu case should run");
    assert!(out.qemu_invoked, "QEMU should be invoked");
    assert_eq!(out.qemu_status, common::QemuStatus::Passed);
    assert_eq!(out.qemu_result, Some(out.semantic_oracle_result));
}

#[test]
fn percentile_calculation_sub() {
    let samples = [1u64, 2, 3, 4, 5];
    assert_eq!(percentile(&samples, 50.0), Some(3));
    assert_eq!(percentile(&[], 50.0), None);
}
