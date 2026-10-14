use std::process::Command;
use std::str;

#[test]
fn real_cli_observes_bound_refill_and_all_outcomes() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-bounded"));
    let output = command.output().expect("Cargo 建好的工具能啟動");
    assert_eq!(output.status.code(), Some(0));
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("報告是 UTF-8");
    assert_eq!(
        stdout,
        "上限 2；追蹤峰值 2；共提交 5\ninput 1：WorkError(Negative(-1))\ninput 2：Panic\ninput 3：Value(200)\ninput 0：Value(80)\ninput 4：Value(30)\n五份工作全部回收；沒有未提交輸入\n"
    );
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("panic 診斷是 UTF-8");
    assert!(stderr.contains("task 故意 panic"));
}
