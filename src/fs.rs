use std::fs;

#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::fs::PermissionsExt;

#[cfg(any(unix, target_os = "redox"))]
pub fn is_executable(md: &fs::Metadata) -> bool {
    md.permissions().mode() & 0o111 != 0
}

#[cfg(windows)]
pub fn is_executable(_: &fs::Metadata) -> bool {
    false
}
