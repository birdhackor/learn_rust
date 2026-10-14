pub fn build_mode() -> &'static str {
    env!("TEACHING_MODE")
}

pub fn target_os() -> &'static str {
    env!("TEACHING_TARGET_OS")
}

pub fn host_windows_text() -> &'static str {
    env!("TEACHING_HOST_WINDOWS")
}
