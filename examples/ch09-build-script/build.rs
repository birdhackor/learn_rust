use std::env;
use std::env::VarError;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=LESSON_MODE");

    let mode = match env::var("LESSON_MODE") {
        Ok(mode) => mode,
        Err(VarError::NotPresent) => {
            let mut default_mode = String::new();
            default_mode.push_str("demo");
            default_mode
        }
        Err(VarError::NotUnicode(_)) => {
            println!("cargo::error=LESSON_MODE 必須是可表示為 Unicode 的 demo 或 practice");
            return;
        }
    };

    if mode.as_str() != "demo" && mode.as_str() != "practice" {
        println!("cargo::error=LESSON_MODE 只接受 demo 或 practice");
        return;
    }

    let target_os =
        env::var("CARGO_CFG_TARGET_OS").expect("Cargo 執行本節 script 時會提供目標作業系統名稱");
    let host_windows = cfg!(target_os = "windows");

    println!("cargo::rustc-env=TEACHING_MODE={mode}");
    println!("cargo::rustc-env=TEACHING_TARGET_OS={target_os}");
    println!("cargo::rustc-env=TEACHING_HOST_WINDOWS={host_windows}");
}
