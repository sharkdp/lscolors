//! A library for colorizing paths according to the `LS_COLORS` environment variable.
//!
//! # Example
//! ```
//! use lscolors::{LsColors, Style};
//!
//! let lscolors = LsColors::from_env().unwrap_or_default();
//!
//! let path = "some/folder/archive.zip";
//! let style = lscolors.style_for_path(path);
//!
//! // If you want to use `ansi_term`:
//! let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
//! println!("{}", ansi_style.paint(path));
//! ```

mod fs;
pub mod style;

use std::env;
use std::path::Path;

pub use crate::style::{Color, FontStyle, Style};

type FileType = String;

const LS_COLORS_DEFAULT: &str = "rs=0:di=01;34:ln=01;36:mh=00:pi=40;33:so=01;35:do=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:mi=00:su=37;41:sg=30;43:ca=30;41:tw=30;42:ow=34;42:st=37;44:ex=01;32:*.tar=01;31:*.tgz=01;31:*.arc=01;31:*.arj=01;31:*.taz=01;31:*.lha=01;31:*.lz4=01;31:*.lzh=01;31:*.lzma=01;31:*.tlz=01;31:*.txz=01;31:*.tzo=01;31:*.t7z=01;31:*.zip=01;31:*.z=01;31:*.dz=01;31:*.gz=01;31:*.lrz=01;31:*.lz=01;31:*.lzo=01;31:*.xz=01;31:*.zst=01;31:*.tzst=01;31:*.bz2=01;31:*.bz=01;31:*.tbz=01;31:*.tbz2=01;31:*.tz=01;31:*.deb=01;31:*.rpm=01;31:*.jar=01;31:*.war=01;31:*.ear=01;31:*.sar=01;31:*.rar=01;31:*.alz=01;31:*.ace=01;31:*.zoo=01;31:*.cpio=01;31:*.7z=01;31:*.rz=01;31:*.cab=01;31:*.wim=01;31:*.swm=01;31:*.dwm=01;31:*.esd=01;31:*.jpg=01;35:*.jpeg=01;35:*.mjpg=01;35:*.mjpeg=01;35:*.gif=01;35:*.bmp=01;35:*.pbm=01;35:*.pgm=01;35:*.ppm=01;35:*.tga=01;35:*.xbm=01;35:*.xpm=01;35:*.tif=01;35:*.tiff=01;35:*.png=01;35:*.svg=01;35:*.svgz=01;35:*.mng=01;35:*.pcx=01;35:*.mov=01;35:*.mpg=01;35:*.mpeg=01;35:*.m2v=01;35:*.mkv=01;35:*.webm=01;35:*.ogm=01;35:*.mp4=01;35:*.m4v=01;35:*.mp4v=01;35:*.vob=01;35:*.qt=01;35:*.nuv=01;35:*.wmv=01;35:*.asf=01;35:*.rm=01;35:*.rmvb=01;35:*.flc=01;35:*.avi=01;35:*.fli=01;35:*.flv=01;35:*.gl=01;35:*.dl=01;35:*.xcf=01;35:*.xwd=01;35:*.yuv=01;35:*.cgm=01;35:*.emf=01;35:*.ogv=01;35:*.ogx=01;35:*.aac=00;36:*.au=00;36:*.flac=00;36:*.m4a=00;36:*.mid=00;36:*.midi=00;36:*.mka=00;36:*.mp3=00;36:*.mpc=00;36:*.ogg=00;36:*.ra=00;36:*.wav=00;36:*.oga=00;36:*.opus=00;36:*.spx=00;36:*.xspf=00;36:";

/// Holds information about how different file system entries should be colorized / styled.
#[derive(Debug, Clone, PartialEq)]
pub struct LsColors {
    // Note: you might expect to see a `HashMap` here, but we need to
    // preserve the exact order of the mapping in order to be consistent
    // with `ls`.
    mapping: Vec<(FileType, Style)>,
    directory: Option<Style>,
    symlink: Option<Style>,
    broken_symlink: Option<Style>,
    executable: Option<Style>,
    fifo: Option<Style>,
    socket: Option<Style>,
    block_device: Option<Style>,
    char_device: Option<Style>,
}

impl Default for LsColors {
    /// Constructs a default `LsColors` instance with some
    /// default styles.
    fn default() -> Self {
        LsColors::from_string(LS_COLORS_DEFAULT)
    }
}

impl LsColors {
    /// Construct an empty [`LsColors`](struct.LsColors.html) instance with no pre-defined styles.
    pub fn empty() -> Self {
        LsColors {
            mapping: vec![],
            directory: None,
            symlink: None,
            broken_symlink: None,
            executable: None,
            fifo: None,
            socket: None,
            block_device: None,
            char_device: None,
        }
    }

    /// Creates a new [`LsColors`](struct.LsColors.html) instance from the `LS_COLORS` environment variable.
    pub fn from_env() -> Option<Self> {
        env::var("LS_COLORS")
            .ok()
            .as_ref()
            .map(|s| Self::from_string(s))
    }

    /// Creates a new [`LsColors`](struct.LsColors.html) instance from the given string.
    pub fn from_string(input: &str) -> Self {
        let mut lscolors = LsColors::empty();

        for entry in input.split(':') {
            let parts: Vec<_> = entry.split('=').collect();

            if let Some([filetype, ansi_style]) = parts.get(0..2) {
                if let Some(style) = Style::from_ansi_sequence(ansi_style) {
                    if filetype.starts_with('*') {
                        lscolors.mapping.push((filetype[1..].to_string(), style));
                    } else {
                        match *filetype {
                            "di" => lscolors.directory = Some(style),
                            "ln" => lscolors.symlink = Some(style),
                            "ex" => lscolors.executable = Some(style),
                            "or" | "mi" => lscolors.broken_symlink = Some(style),
                            "pi" => lscolors.fifo = Some(style),
                            "so" => lscolors.socket = Some(style),
                            "bd" => lscolors.block_device = Some(style),
                            "cd" => lscolors.char_device = Some(style),
                            _ => {}
                        }
                    }
                }
            }
        }

        lscolors
    }

    /// Get the ANSI style for a given path.
    ///
    /// *Note:* this function calls `Path::symlink_metadata` internally. If you already happen to
    /// have the `Metadata` available, use [`style_for_path_with_metadata`](#method.style_for_path_with_metadata).
    pub fn style_for_path<P: AsRef<Path>>(&self, path: P) -> Option<&Style> {
        let metadata = path.as_ref().symlink_metadata().ok();
        self.style_for_path_with_metadata(path, metadata.as_ref())
    }

    /// Get the ANSI style for a path, given the corresponding `Metadata` struct.
    ///
    /// *Note:* The `Metadata` struct must have been acquired via `Path::symlink_metadata` in
    /// order to colorize symbolic links correctly.
    pub fn style_for_path_with_metadata<P: AsRef<Path>>(
        &self,
        path: P,
        metadata: Option<&std::fs::Metadata>,
    ) -> Option<&Style> {
        if let Some(metadata) = metadata {
            if metadata.is_dir() {
                return self.directory.as_ref();
            }

            if metadata.file_type().is_symlink() {
                // This works because `Path::exists` traverses symlinks.
                if path.as_ref().exists() {
                    return self.symlink.as_ref();
                } else {
                    return self.broken_symlink.as_ref();
                }
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::FileTypeExt;

                let filetype = metadata.file_type();
                if filetype.is_fifo() {
                    return self.fifo.as_ref();
                }
                if filetype.is_socket() {
                    return self.socket.as_ref();
                }
                if filetype.is_block_device() {
                    return self.block_device.as_ref();
                }
                if filetype.is_char_device() {
                    return self.char_device.as_ref();
                }
            }

            if crate::fs::is_executable(&metadata) {
                return self.executable.as_ref();
            }
        }

        // Note: using '.to_str()' here means that filename
        // matching will not work with invalid-UTF-8 paths.
        let filename = path.as_ref().file_name()?.to_str()?;

        // We need to traverse LS_COLORS from back to front
        // to be consistent with `ls`:
        for (filetype, style) in self.mapping.iter().rev() {
            // Note: For some reason, 'ends_with' is much
            // slower if we omit `.as_str()` here:
            if filename.ends_with(filetype.as_str()) {
                return Some(style);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, FontStyle};
    use crate::{LsColors, LS_COLORS_DEFAULT};

    #[test]
    fn basic_usage() {
        let lscolors = LsColors::from_string(LS_COLORS_DEFAULT);

        let style_dir = lscolors.directory.clone().unwrap();
        assert_eq!(FontStyle::bold(), style_dir.font_style);
        assert_eq!(Some(Color::Blue), style_dir.foreground);
        assert_eq!(None, style_dir.background);

        let style_symlink = lscolors.symlink.clone().unwrap();
        assert_eq!(FontStyle::bold(), style_symlink.font_style);
        assert_eq!(Some(Color::Cyan), style_symlink.foreground);
        assert_eq!(None, style_symlink.background);

        let style_rs = lscolors.style_for_path("test.wav").unwrap();
        assert_eq!(FontStyle::default(), style_rs.font_style);
        assert_eq!(Some(Color::Cyan), style_rs.foreground);
        assert_eq!(None, style_rs.background);
    }

    #[test]
    fn get_style_for_path_uses_correct_ordering() {
        let lscolors = LsColors::from_string("*.foo=01;35:*README.foo=33;44");

        let style_foo = lscolors.style_for_path("some/folder/dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);
        assert_eq!(None, style_foo.background);

        let style_readme = lscolors
            .style_for_path("some/other/folder/README.foo")
            .unwrap();
        assert_eq!(FontStyle::default(), style_readme.font_style);
        assert_eq!(Some(Color::Yellow), style_readme.foreground);
        assert_eq!(Some(Color::Blue), style_readme.background);
    }
}
