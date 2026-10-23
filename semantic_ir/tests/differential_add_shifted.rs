mod common;

use common::{percentile, qemu_available, run_differential_suite};

#[test]
fn differential_add_shifted_suite() {
    let cases: [(u64, u64); 6] = [
        (0, 0),
        (1, 0),
        (0, 5),
        (10, 20),
        (u64::MAX, 1),
        (u64::MAX / 2 + 1, u64::MAX / 2 + 1),
    ];
    let mismatches = run_differential_suite(&cases).expect("differential suite should run");
    assert!(
        mismatches.is_empty(),
        "differential mismatches: {:?}",
        mismatches
    );

    // Prove QEMU actually executed: run one case and check status.
    let out = common::run_differential_case_op(common::ArithmeticOp::Add, cases[0].0, cases[0].1)
        .expect("qemu case should run");
    assert!(out.qemu_invoked, "QEMU should be invoked");
    assert_eq!(out.qemu_status, common::QemuStatus::Passed);
    assert_eq!(out.qemu_result, Some(out.semantic_oracle_result));


}

#[test]
fn percentile_calculation() {
    let samples = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(percentile(&samples, 50.0), Some(5));
    assert_eq!(percentile(&samples, 95.0), Some(10));
    assert_eq!(percentile(&[], 50.0), None);
}
