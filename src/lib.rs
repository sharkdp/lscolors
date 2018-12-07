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
