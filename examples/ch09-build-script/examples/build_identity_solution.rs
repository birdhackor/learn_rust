use ch09_build_script::build_mode;
use ch09_build_script::target_os;

fn main() {
    let package = env!("CARGO_PKG_NAME");
    let identity = format!(
        "package={package}; mode={}; target={}",
        build_mode(),
        target_os()
    );
    println!("{identity}");
}
