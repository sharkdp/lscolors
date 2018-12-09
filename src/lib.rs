//! This crate contains datatypes and functions for working with the `LS_COLORS` environment
//! variable.

mod fs;
pub mod style;

use std::path::Path;

use crate::style::Style;

// TODO: do we need this?
// const LS_CODES: &[&str] = &[
//     "no", "no", "fi", "rs", "di", "ln", "ln", "ln", "or", "mi", "pi", "pi", "so", "bd", "bd", "cd",
//     "cd", "do", "ex", "lc", "lc", "rc", "rc", "ec", "ec", "su", "su", "sg", "sg", "st", "ow", "ow",
//     "tw", "tw", "ca", "mh", "cl",
// ];

type FileType<'a> = &'a str;

/// Defines how different file system entries should be colorized / styled.
#[derive(Debug, PartialEq)]
pub struct LsColors<'a> {
    mapping: Vec<(FileType<'a>, Style)>,
    directory: Option<Style>,
    symlink: Option<Style>,
    executable: Option<Style>,
}

impl<'a> Default for LsColors<'a> {
    fn default() -> Self {
        // TODO: do we want to return some pre-filled default here?
        LsColors::empty()
    }
}

impl<'a> LsColors<'a> {
    pub fn empty() -> Self {
        LsColors {
            mapping: vec![],
            directory: None,
            symlink: None,
            executable: None,
        }
    }

    pub fn from_string(input: &'a str) -> Self {
        let mut lscolors = LsColors::empty();

        for entry in input.split(":") {
            let parts: Vec<_> = entry.split('=').collect();

            if let Some([filetype, ansi_style]) = parts.get(0..2) {
                if let Some(style) = Style::from_ansi_sequence(ansi_style) {
                    if filetype.starts_with("*") {
                        lscolors.mapping.push((&filetype[1..], style));
                    } else {
                        match filetype {
                            &"di" => lscolors.directory = Some(style),
                            &"ln" => lscolors.symlink = Some(style),
                            &"ex" => lscolors.executable = Some(style),
                            _ => {}
                        }
                    }
                }
            }
        }

        lscolors
    }

    // TODO: write an alternative function which does not call metadata().
    pub fn get_style_for<P: AsRef<Path>>(&self, path: P) -> Option<&Style> {
        if let Ok(metadata) = path.as_ref().symlink_metadata() {
            if metadata.is_dir() {
                return self.directory.as_ref();
            } else if metadata.file_type().is_symlink() {
                return self.symlink.as_ref();
            } else if crate::fs::is_executable(&metadata) {
                return self.executable.as_ref();
            }
        }

        // TODO: avoid the costly (?) 'to_str' call here which
        // needs to check for UTF-8 validity. Also, this does not
        // work with invalid-UTF-8 paths.
        let filename = path.as_ref().file_name()?.to_str()?;

        // We need to traverse LS_COLORS from back to front
        // to be consistent with `ls`:
        for (filetype, style) in self.mapping.iter().rev() {
            if filename.ends_with(filetype) {
                return Some(style);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, FontStyle};
    use crate::LsColors;

    const LS_COLORS_DEFAULT: &str = "rs=0:di=01;34:ln=01;36:mh=00:pi=40;33:so=01;35:do=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:mi=00:su=37;41:sg=30;43:ca=30;41:tw=30;42:ow=34;42:st=37;44:ex=01;32:*.tar=01;31:*.tgz=01;31:*.arc=01;31:*.arj=01;31:*.taz=01;31:*.lha=01;31:*.lz4=01;31:*.lzh=01;31:*.lzma=01;31:*.tlz=01;31:*.txz=01;31:*.tzo=01;31:*.t7z=01;31:*.zip=01;31:*.z=01;31:*.dz=01;31:*.gz=01;31:*.lrz=01;31:*.lz=01;31:*.lzo=01;31:*.xz=01;31:*.zst=01;31:*.tzst=01;31:*.bz2=01;31:*.bz=01;31:*.tbz=01;31:*.tbz2=01;31:*.tz=01;31:*.deb=01;31:*.rpm=01;31:*.jar=01;31:*.war=01;31:*.ear=01;31:*.sar=01;31:*.rar=01;31:*.alz=01;31:*.ace=01;31:*.zoo=01;31:*.cpio=01;31:*.7z=01;31:*.rz=01;31:*.cab=01;31:*.wim=01;31:*.swm=01;31:*.dwm=01;31:*.esd=01;31:*.jpg=01;35:*.jpeg=01;35:*.mjpg=01;35:*.mjpeg=01;35:*.gif=01;35:*.bmp=01;35:*.pbm=01;35:*.pgm=01;35:*.ppm=01;35:*.tga=01;35:*.xbm=01;35:*.xpm=01;35:*.tif=01;35:*.tiff=01;35:*.png=01;35:*.svg=01;35:*.svgz=01;35:*.mng=01;35:*.pcx=01;35:*.mov=01;35:*.mpg=01;35:*.mpeg=01;35:*.m2v=01;35:*.mkv=01;35:*.webm=01;35:*.ogm=01;35:*.mp4=01;35:*.m4v=01;35:*.mp4v=01;35:*.vob=01;35:*.qt=01;35:*.nuv=01;35:*.wmv=01;35:*.asf=01;35:*.rm=01;35:*.rmvb=01;35:*.flc=01;35:*.avi=01;35:*.fli=01;35:*.flv=01;35:*.gl=01;35:*.dl=01;35:*.xcf=01;35:*.xwd=01;35:*.yuv=01;35:*.cgm=01;35:*.emf=01;35:*.ogv=01;35:*.ogx=01;35:*.aac=00;36:*.au=00;36:*.flac=00;36:*.m4a=00;36:*.mid=00;36:*.midi=00;36:*.mka=00;36:*.mp3=00;36:*.mpc=00;36:*.ogg=00;36:*.ra=00;36:*.wav=00;36:*.oga=00;36:*.opus=00;36:*.spx=00;36:*.xspf=00;36:";

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

        let style_rs = lscolors.get_style_for("test.wav").unwrap();
        assert_eq!(FontStyle::default(), style_rs.font_style);
        assert_eq!(Some(Color::Cyan), style_rs.foreground);
        assert_eq!(None, style_rs.background);
    }

    #[test]
    fn uses_correct_ordering() {
        let lscolors =
            LsColors::from_string("rs=0:di=03;34:ln=01;36:*.foo=01;35:*README.foo=33;44");

        let style_foo = lscolors.get_style_for("dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);
        assert_eq!(None, style_foo.background);

        let style_readme = lscolors.get_style_for("some/folder/README.foo").unwrap();
        assert_eq!(FontStyle::default(), style_readme.font_style);
        assert_eq!(Some(Color::Yellow), style_readme.foreground);
        assert_eq!(Some(Color::Blue), style_readme.background);
    }
}
