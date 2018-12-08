/// A parser for the `LS_COLORS` environment variable.
use std::collections::HashMap;

pub mod style;

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
    mapping: HashMap<FileType<'a>, Style>,
}

impl<'a> LsColors<'a> {
    pub fn from_string(lscolors: &'a str) -> Self {
        let mut mapping = HashMap::new();

        for entry in lscolors.split(":") {
            let parts: Vec<_> = entry.split('=').collect();

            if let Some([filetype, ansi_style]) = parts.get(0..2) {
                if let Some(style) = Style::from_ansi_sequence(ansi_style) {
                    if filetype.starts_with("*") {
                        mapping.insert(&filetype[1..], style);
                    } else {
                        let result = LS_CODES.iter().find(|&c| c == filetype);

                        if let Some(code) = result {
                            match code {
                                // "di" => self.directory = style,
                                // "ln" => self.symlink = style,
                                // "ex" => self.executable = style,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        LsColors { mapping }
    }

    pub fn get_style_for(&self, filename: &str) -> Option<&Style> {
        for i in 0..(filename.len() - 1) {
            if let Some(style) = self.mapping.get(&filename[i..]) {
                return Some(style);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::style::{Color, FontStyle};
    use crate::LsColors;

    #[test]
    fn test_from_string() {
        let result = LsColors::from_string("rs=0:di=03;34:ln=01;36:*.foo=01;35:*README.foo=33");

        let style_foo = result.get_style_for("dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);

        let style_foo = result.get_style_for("README.foo").unwrap();
        assert_eq!(FontStyle::default(), style_foo.font_style);
        assert_eq!(Some(Color::Yellow), style_foo.foreground);

        // TODO: tests for directory, etc.
    }

    // TODO: tests for invalid patterns
}
