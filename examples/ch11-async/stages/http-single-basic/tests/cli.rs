use std::process::Command;
use std::process::Output;
use std::str;

fn execute(arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ch11-http-single-basic"));
    for argument in arguments {
        command.arg(argument);
    }
    command.output().expect("Cargo 建好的工具可以啟動")
}

#[test]
fn help_and_version_succeed_without_sending_a_request() {
    let help = execute(&["--help"][..]);
    assert_eq!(help.status.code(), Some(0));
    assert_eq!(help.stderr.len(), 0);
    let text = str::from_utf8(help.stdout.as_slice()).expect("help 是 UTF-8");
    assert!(text.starts_with("以單一 async 工作檢查 HTTP(S) URL"));
    let version = execute(&["--version"][..]);
    assert_eq!(version.status.code(), Some(0));
    assert_eq!(version.stdout.as_slice(), b"async-http-inspect 0.1.0\n");
    assert_eq!(version.stderr.len(), 0);
}

#[test]
fn invalid_arguments_and_urls_exit_two_without_a_report() {
    for arguments in [
        &[][..],
        &["--unknown"][..],
        &["/notes"][..],
        &["file:///tmp/notes.txt"][..],
    ] {
        let output = execute(arguments);
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stdout.len(), 0);
        assert!(!output.stderr.as_slice().is_empty());
    }
}
