use std::process::Command;
use std::str;

#[test]
fn real_cli_reports_original_inputs_and_finishes_despite_task_panic() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-tasks"));
    let output = command.output().expect("Cargo 建好的工具能啟動");
    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("報告是 UTF-8");
    assert_eq!(
        stdout,
        "input 1：WorkError(Negative(-1))\ninput 2：Panic\ninput 3：Cancelled\ninput 0：Value(80)\n四份 task 都已回收\n"
    );
    let stderr = str::from_utf8(output.stderr.as_slice()).expect("panic 診斷是 UTF-8");
    assert!(stderr.contains("task 故意 panic"));
}
