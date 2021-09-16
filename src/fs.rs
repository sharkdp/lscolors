use std::fs;

#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::fs::MetadataExt;

/// Get the UNIX-style mode bits from some metadata if available, otherwise 0.
#[allow(unused_variables)]
pub fn mode(md: &fs::Metadata) -> u32 {
    #[cfg(any(unix, target_os = "redox"))]
    return md.mode();

    #[cfg(not(any(unix, target_os = "redox")))]
    return 0;
}

/// Get the number of hard links to a file, or 1 if unknown.
#[allow(unused_variables)]
pub fn nlink(md: &fs::Metadata) -> u64 {
    #[cfg(any(unix, target_os = "redox"))]
    return md.nlink();

    #[cfg(not(any(unix, target_os = "redox")))]
    return 1;
}
