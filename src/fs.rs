use std::fs;

#[cfg(any(unix, target_os = "redox"))]
pub fn is_executable(md: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    md.permissions().mode() & 0o111 != 0
}

#[cfg(any(windows, target_os = "wasi"))]
pub fn is_executable(_: &fs::Metadata) -> bool {
    false
}
