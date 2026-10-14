use ch09_build_script::build_mode;
use ch09_build_script::target_os;

#[test]
fn compiled_mode_is_one_of_the_accepted_values() {
    let mode = build_mode();
    if mode != "demo" {
        assert_eq!(mode, "practice");
    }
}

#[cfg(target_os = "linux")]
#[test]
fn cargo_target_information_matches_the_compiled_crate() {
    assert_eq!(target_os(), "linux");
}

#[cfg(target_os = "windows")]
#[test]
fn cargo_target_information_matches_the_compiled_crate() {
    assert_eq!(target_os(), "windows");
}
