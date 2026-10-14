use std::process::Command;
use std::str;

#[test]
fn real_cli_distinguishes_construction_execution_and_discarding() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-foundations"));
    let output = command.output().expect("Cargo 建好的範例能啟動");
    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr.len(), 0);
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("本例輸出是 UTF-8");
    assert_eq!(
        stdout,
        "普通函式已交回：100\nfuture 已建立，async 本體尚未執行\nasync 本體：處理小計 80\nawait 已取得結果：100\n另一份 future 未執行就已丟棄\n"
    );
}
