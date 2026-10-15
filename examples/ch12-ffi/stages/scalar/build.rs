fn main() {
    println!("cargo::rerun-if-changed=c/tutorial.c");
    println!("cargo::rerun-if-changed=c/tutorial.h");
    cc::Build::new()
        .file("c/tutorial.c")
        .std("c11")
        .warnings(true)
        .warnings_into_errors(true)
        .compile("tutorial_c");
}
