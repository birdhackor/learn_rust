use ch09_build_script::build_mode;
use ch09_build_script::host_windows_text;
use ch09_build_script::target_os;

fn main() {
    println!("建置時選的模式={}", build_mode());
    println!("主要 crate 的 target OS={}", target_os());
    println!("script 的 host 是 Windows={}", host_windows_text());
}
