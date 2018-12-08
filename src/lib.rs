/// This crate contains datatypes and functions to work with the `LS_COLORS` environment variable.
pub mod style;

use std::path::Path;

use crate::style::Style;

const LS_CODES: &[&str] = &[
    "no", "no", "fi", "rs", "di", "ln", "ln", "ln", "or", "mi", "pi", "pi", "so", "bd", "bd", "cd",
    "cd", "do", "ex", "lc", "lc", "rc", "rc", "ec", "ec", "su", "su", "sg", "sg", "st", "ow", "ow",
    "tw", "tw", "ca", "mh", "cl",
];

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
        LsColors {
            mapping: vec![],
            directory: None,
            symlink: None,
            executable: None,
        }
    }
}

impl<'a> LsColors<'a> {
    pub fn from_string(input: &'a str) -> Self {
        let mut lscolors = LsColors::default();

        for entry in input.split(":") {
            let parts: Vec<_> = entry.split('=').collect();

            if let Some([filetype, ansi_style]) = parts.get(0..2) {
                if let Some(style) = Style::from_ansi_sequence(ansi_style) {
                    if filetype.starts_with("*") {
                        lscolors.mapping.push((&filetype[1..], style));
                    } else {
                        let result = LS_CODES.iter().find(|&c| c == filetype);

                        if result.is_some() {
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
            }
            // TODO: executable
        }

        // TODO: avoid the costly (?) 'to_str' call here which
        // needs to check for UTF-8 validity. Also, this does not
        // work with invalid-UTF-8 paths.
        let filename = path.as_ref().file_name()?.to_str()?;

        for (filetype, style) in &self.mapping {
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

    #[test]
    fn test_from_string() {
        let lscolors =
            LsColors::from_string("rs=0:di=03;34:ln=01;36:*README.foo=33;44:*.foo=01;35");

        let style_foo = lscolors.get_style_for("dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);
        assert_eq!(None, style_foo.background);

        let style_readme = lscolors.get_style_for("some/folder/README.foo").unwrap();
        assert_eq!(FontStyle::default(), style_readme.font_style);
        assert_eq!(Some(Color::Yellow), style_readme.foreground);
        assert_eq!(Some(Color::Blue), style_readme.background);

        // TODO: tests for directory, etc.
    }

    // TODO: tests for invalid patterns
}
