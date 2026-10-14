use std::process::Command;
use std::str;

#[test]
fn the_finite_demo_reports_a_real_released_response_and_server_finish() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-local-service"));
    let output = command.output().expect("Cargo 建好的有限示範可啟動");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr.len(), 0);
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("示範是 UTF-8");
    assert_eq!(
        stdout,
        "服務已收到 input 0；尚未交回 headers\n放行後：HTTP 200，body=Some(\"held 已放行\")\n已等待 server；共處理 1 份工作請求\n"
    );
}
