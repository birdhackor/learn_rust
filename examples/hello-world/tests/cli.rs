use std::process::Command;

#[test]
fn prints_the_tutorial_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_hello-world"))
        .output()
        .expect("範例程式應可執行");

    assert!(output.status.success(), "範例程式應成功結束");
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, Rust!\n");
    assert!(output.stderr.is_empty());
}
