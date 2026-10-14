use std::process::Command;
use std::process::Output;
use std::str;

fn execute(arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-http"));
    for argument in arguments {
        command.arg(argument);
    }
    command.output().expect("Cargo 建好的工具能啟動")
}

#[test]
fn help_is_success_without_a_url_or_a_network_request() {
    let output = execute(&["--help"][..]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stderr.len(), 0);
    let stdout = str::from_utf8(output.stdout.as_slice()).expect("help 是 UTF-8");
    assert!(stdout.starts_with("檢查一個 HTTP(S) URL"));
}

#[test]
fn input_errors_have_status_two_and_no_report() {
    for arguments in [&["/notes"][..], &["file:///tmp/notes.txt"][..], &[][..]] {
        let output = execute(arguments);
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stdout.len(), 0);
        assert!(!output.stderr.as_slice().is_empty());
    }
}
