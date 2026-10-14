use std::process::Command;
use std::str;

#[test]
fn controlled_cli_keeps_failures_and_classifies_all_inputs() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-cancellation"));
    let output = command.output().expect("Cargo 建好的工具能啟動");
    assert_eq!(output.status.code(), Some(130));
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("報告是 UTF-8");
    assert_eq!(
        stdout,
        "結束原因：Requested\n共提交 4；追蹤峰值 2\ninput 0：Cancelled\ninput 1：WorkError(Negative(-1))\ninput 2：Panic\ninput 3：Cancelled\ninput 4：Unstarted\n已回收全部提交工作，並清理未提交輸入\n"
    );
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("panic 診斷是 UTF-8");
    assert!(stderr.contains("task 故意 panic"));
}

#[test]
fn invalid_mode_is_rejected_before_signal_or_tasks_start() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-cancellation"));
    command.arg("--unknown");
    let output = command.output().expect("Cargo 建好的工具能啟動");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.len(), 0);
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("用法是 UTF-8");
    assert_eq!(stderr, "用法：ch11-cancellation [--signal]\n");
}
