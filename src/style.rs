//! A module for ANSI styles
//!
//! For more information, see
//! [ANSI escape code (Wikipedia)](https://en.wikipedia.org/wiki/ANSI_escape_code).
#[cfg(feature = "nu-ansi-term")]
use nu_ansi_term;
use std::collections::VecDeque;

/// A `Color` can be one of the pre-defined ANSI colors (`Red`, `Green`, ..),
/// a 8-bit ANSI color (`Fixed(u8)`) or a 24-bit color (`RGB(u8, u8, u8)`).
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Fixed(u8),
    RGB(u8, u8, u8),
}

impl Color {
    /// Convert to a `ansi_term::Color` (if the `ansi_term` feature is enabled).
    #[cfg(feature = "ansi_term")]
    pub fn to_ansi_term_color(&self) -> ansi_term::Color {
        match self {
            Color::RGB(r, g, b) => ansi_term::Color::RGB(*r, *g, *b),
            Color::Fixed(n) => ansi_term::Color::Fixed(*n),
            Color::Black => ansi_term::Color::Black,
            Color::Red => ansi_term::Color::Red,
            Color::Green => ansi_term::Color::Green,
            Color::Yellow => ansi_term::Color::Yellow,
            Color::Blue => ansi_term::Color::Blue,
            Color::Magenta => ansi_term::Color::Purple,
            Color::Cyan => ansi_term::Color::Cyan,
            Color::White => ansi_term::Color::White,

            // Below items are a rough translations to 256 colors as
            // we do not have bright varients available on ansi-term
            Color::BrightBlack => ansi_term::Color::Fixed(8),
            Color::BrightRed => ansi_term::Color::Fixed(9),
            Color::BrightGreen => ansi_term::Color::Fixed(10),
            Color::BrightYellow => ansi_term::Color::Fixed(11),
            Color::BrightBlue => ansi_term::Color::Fixed(12),
            Color::BrightMagenta => ansi_term::Color::Fixed(13),
            Color::BrightCyan => ansi_term::Color::Fixed(14),
            Color::BrightWhite => ansi_term::Color::Fixed(15),
        }
    }

    /// Convert to a `nu_ansi_term::Color` (if the `nu_ansi_term` or `gnu_legacy` feature is enabled).
    #[cfg(any(feature = "nu-ansi-term", feature = "gnu_legacy"))]
    pub fn to_nu_ansi_term_color(&self) -> nu_ansi_term::Color {
        match self {
            Color::RGB(r, g, b) => nu_ansi_term::Color::Rgb(*r, *g, *b),
            Color::Fixed(n) => nu_ansi_term::Color::Fixed(*n),
            Color::Black => nu_ansi_term::Color::Black,
            Color::Red => nu_ansi_term::Color::Red,
            Color::Green => nu_ansi_term::Color::Green,
            Color::Yellow => nu_ansi_term::Color::Yellow,
            Color::Blue => nu_ansi_term::Color::Blue,
            Color::Magenta => nu_ansi_term::Color::Purple,
            Color::Cyan => nu_ansi_term::Color::Cyan,
            Color::White => nu_ansi_term::Color::White,

            // Below items are a rough translations to 256 colors as
            // we do not have bright varients available on ansi-term
            Color::BrightBlack => nu_ansi_term::Color::Fixed(8),
            Color::BrightRed => nu_ansi_term::Color::Fixed(9),
            Color::BrightGreen => nu_ansi_term::Color::Fixed(10),
            Color::BrightYellow => nu_ansi_term::Color::Fixed(11),
            Color::BrightBlue => nu_ansi_term::Color::Fixed(12),
            Color::BrightMagenta => nu_ansi_term::Color::Fixed(13),
            Color::BrightCyan => nu_ansi_term::Color::Fixed(14),
            Color::BrightWhite => nu_ansi_term::Color::Fixed(15),
        }
    }

    /// Convert to a `crossterm::style::Color` (if the `crossterm` feature is enabled).
    #[cfg(feature = "crossterm")]
    pub fn to_crossterm_color(&self) -> crossterm::style::Color {
        match self {
            Color::RGB(r, g, b) => crossterm::style::Color::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
            Color::Fixed(n) => crossterm::style::Color::AnsiValue(*n),
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::DarkRed,
            Color::Green => crossterm::style::Color::DarkGreen,
            Color::Yellow => crossterm::style::Color::DarkYellow,
            Color::Blue => crossterm::style::Color::DarkBlue,
            Color::Magenta => crossterm::style::Color::DarkMagenta,
            Color::Cyan => crossterm::style::Color::DarkCyan,
            Color::White => crossterm::style::Color::Grey,
            Color::BrightBlack => crossterm::style::Color::DarkGrey,
            Color::BrightRed => crossterm::style::Color::Red,
            Color::BrightGreen => crossterm::style::Color::Green,
            Color::BrightYellow => crossterm::style::Color::Yellow,
            Color::BrightBlue => crossterm::style::Color::Blue,
            Color::BrightMagenta => crossterm::style::Color::Magenta,
            Color::BrightCyan => crossterm::style::Color::Cyan,
            Color::BrightWhite => crossterm::style::Color::White,
        }
    }
}

/// Font-style attributes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontStyle {
    pub bold: bool,
    pub dimmed: bool, // a.k.a. faint
    pub italic: bool,
    pub underline: bool,
    pub slow_blink: bool,
    pub rapid_blink: bool,
    pub reverse: bool,       // a.k.a. inverse or reverse video
    pub hidden: bool,        // a.k.a. conceal
    pub strikethrough: bool, // a.k.a. crossed-out
}

impl FontStyle {
    pub fn bold() -> Self {
        FontStyle {
            bold: true,
            ..Default::default()
        }
    }

    pub fn dimmed() -> Self {
        FontStyle {
            dimmed: true,
            ..Default::default()
        }
    }

    pub fn italic() -> Self {
        FontStyle {
            italic: true,
            ..Default::default()
        }
    }

    pub fn underline() -> Self {
        FontStyle {
            underline: true,
            ..Default::default()
        }
    }

    pub fn slow_blink() -> Self {
        FontStyle {
            slow_blink: true,
            ..Default::default()
        }
    }

    pub fn rapid_blink() -> Self {
        FontStyle {
            rapid_blink: true,
            ..Default::default()
        }
    }

    pub fn reverse() -> Self {
        FontStyle {
            reverse: true,
            ..Default::default()
        }
    }

    pub fn hidden() -> Self {
        FontStyle {
            hidden: true,
            ..Default::default()
        }
    }

    pub fn strikethrough() -> Self {
        FontStyle {
            strikethrough: true,
            ..Default::default()
        }
    }

    /// Convert to `crossterm::style::Attributes` (if the `crossterm` feature is enabled).
    #[cfg(feature = "crossterm")]
    pub fn to_crossterm_attributes(&self) -> crossterm::style::Attributes {
        let mut attributes = crossterm::style::Attributes::default();
        if self.bold {
            attributes.set(crossterm::style::Attribute::Bold);
        }
        if self.dimmed {
            attributes.set(crossterm::style::Attribute::Dim);
        }
        if self.italic {
            attributes.set(crossterm::style::Attribute::Italic);
        }
        if self.underline {
            attributes.set(crossterm::style::Attribute::Underlined);
        }
        if self.slow_blink {
            attributes.set(crossterm::style::Attribute::SlowBlink);
        }
        if self.rapid_blink {
            attributes.set(crossterm::style::Attribute::RapidBlink);
        }
        if self.reverse {
            attributes.set(crossterm::style::Attribute::Reverse);
        }
        if self.hidden {
            attributes.set(crossterm::style::Attribute::Hidden);
        }
        if self.strikethrough {
            attributes.set(crossterm::style::Attribute::CrossedOut);
        }
        attributes
    }
}

/// A foreground color, background color and font-style.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub font_style: FontStyle,
    pub underline: Option<Color>,
}

impl Style {
    /// Parse ANSI escape sequences like `38;2;255;0;100;1;4` (pink, bold, underlined).
    pub fn from_ansi_sequence(code: &str) -> Option<Style> {
        if code.is_empty() || code == "0" || code == "00" {
            return None;
        }

        let mut parts: VecDeque<u8> = code
            .split(';')
            .map(|c| c.parse::<u8>().ok())
            .collect::<Option<_>>()?;

        let mut font_style = FontStyle::default();
        let mut foreground = None;
        let mut background = None;
        let mut underline = None;

        loop {
            match parts.pop_front() {
                Some(0) => font_style = FontStyle::default(),
                Some(1) => font_style.bold = true,
                Some(2) => font_style.dimmed = true,
                Some(3) => font_style.italic = true,
                Some(4) => font_style.underline = true,
                Some(5) => font_style.slow_blink = true,
                Some(6) => font_style.rapid_blink = true,
                Some(7) => font_style.reverse = true,
                Some(8) => font_style.hidden = true,
                Some(9) => font_style.strikethrough = true,
                Some(22) => {
                    font_style.bold = false;
                    font_style.dimmed = false;
                }
                Some(23) => {
                    font_style.italic = false;
                }
                Some(24) => {
                    font_style.underline = false;
                }
                Some(25) => {
                    font_style.slow_blink = false;
                    font_style.rapid_blink = false;
                }
                Some(27) => {
                    font_style.reverse = false;
                }
                Some(28) => {
                    font_style.hidden = false;
                }
                Some(29) => {
                    font_style.strikethrough = false;
                }
                Some(30) => foreground = Some(Color::Black),
                Some(31) => foreground = Some(Color::Red),
                Some(32) => foreground = Some(Color::Green),
                Some(33) => foreground = Some(Color::Yellow),
                Some(34) => foreground = Some(Color::Blue),
                Some(35) => foreground = Some(Color::Magenta),
                Some(36) => foreground = Some(Color::Cyan),
                Some(37) => foreground = Some(Color::White),
                Some(38) => match (parts.pop_front(), parts.pop_front()) {
                    (Some(5), Some(color)) => foreground = Some(Color::Fixed(color)),
                    (Some(2), Some(red)) => match (parts.pop_front(), parts.pop_front()) {
                        (Some(green), Some(blue)) => {
                            foreground = Some(Color::RGB(red, green, blue))
                        }
                        _ => {
                            break;
                        }
                    },
                    _ => {
                        break;
                    }
                },
                Some(39) => foreground = None,
                Some(40) => background = Some(Color::Black),
                Some(41) => background = Some(Color::Red),
                Some(42) => background = Some(Color::Green),
                Some(43) => background = Some(Color::Yellow),
                Some(44) => background = Some(Color::Blue),
                Some(45) => background = Some(Color::Magenta),
                Some(46) => background = Some(Color::Cyan),
                Some(47) => background = Some(Color::White),
                Some(48) => match (parts.pop_front(), parts.pop_front()) {
                    (Some(5), Some(color)) => background = Some(Color::Fixed(color)),
                    (Some(2), Some(red)) => match (parts.pop_front(), parts.pop_front()) {
                        (Some(green), Some(blue)) => {
                            background = Some(Color::RGB(red, green, blue))
                        }
                        _ => {
                            break;
                        }
                    },
                    _ => {
                        break;
                    }
                },
                Some(49) => background = None,
                Some(58) => match (parts.pop_front(), parts.pop_front()) {
                    (Some(5), Some(color)) => underline = Some(Color::Fixed(color)),
                    (Some(2), Some(red)) => match (parts.pop_front(), parts.pop_front()) {
                        (Some(green), Some(blue)) => underline = Some(Color::RGB(red, green, blue)),
                        _ => {
                            break;
                        }
                    },
                    _ => {
                        break;
                    }
                },
                Some(59) => underline = None,
                Some(90) => foreground = Some(Color::BrightBlack),
                Some(91) => foreground = Some(Color::BrightRed),
                Some(92) => foreground = Some(Color::BrightGreen),
                Some(93) => foreground = Some(Color::BrightYellow),
                Some(94) => foreground = Some(Color::BrightBlue),
                Some(95) => foreground = Some(Color::BrightMagenta),
                Some(96) => foreground = Some(Color::BrightCyan),
                Some(97) => foreground = Some(Color::BrightWhite),
                Some(100) => background = Some(Color::BrightBlack),
                Some(101) => background = Some(Color::BrightRed),
                Some(102) => background = Some(Color::BrightGreen),
                Some(103) => background = Some(Color::BrightYellow),
                Some(104) => background = Some(Color::BrightBlue),
                Some(105) => background = Some(Color::BrightMagenta),
                Some(106) => background = Some(Color::BrightCyan),
                Some(107) => background = Some(Color::BrightWhite),
                Some(_) => {
                    continue;
                }
                None => {
                    break;
                }
            }
        }

        Some(Style {
            foreground,
            background,
            font_style,
            underline,
        })
    }

    /// Convert to a `ansi_term::Style` (if the `ansi_term` feature is enabled).
    #[cfg(feature = "ansi_term")]
    pub fn to_ansi_term_style(&self) -> ansi_term::Style {
        ansi_term::Style {
            foreground: self.foreground.as_ref().map(Color::to_ansi_term_color),
            background: self.background.as_ref().map(Color::to_ansi_term_color),
            is_bold: self.font_style.bold,
            is_dimmed: self.font_style.dimmed,
            is_italic: self.font_style.italic,
            is_underline: self.font_style.underline,
            is_blink: self.font_style.rapid_blink || self.font_style.slow_blink,
            is_reverse: self.font_style.reverse,
            is_hidden: self.font_style.hidden,
            is_strikethrough: self.font_style.strikethrough,
        }
    }

    /// Convert to a `nu_ansi_term::Style` (if the `nu-ansi-term` or `gnu_legacy` feature is enabled).
    ///
    /// ## Example for nu-ansi-term feature
    /// ```
    /// # #[cfg(all(feature = "nu-ansi-term", not(feature = "gnu_legacy")))]
    /// # {
    ///
    /// use lscolors::{Color, FontStyle, Style};
    ///
    /// let style = Style {
    ///     font_style: FontStyle {
    ///         bold: true,
    ///         ..Default::default()
    ///     },
    ///     foreground: Some(Color::Blue),
    ///     ..Default::default()
    /// };
    /// let nu_ansi = style.to_nu_ansi_term_style();
    /// assert_eq!("\x1b[1;34mwow\x1b[0m", nu_ansi.paint("wow").to_string());
    /// # }
    /// ```
    /// ## Example for gnu_legacy feature
    /// ```
    /// # #[cfg(feature = "gnu_legacy")]
    /// # {
    ///
    /// use lscolors::{Color, FontStyle, Style};
    ///
    /// let style = Style {
    ///     font_style: FontStyle {
    ///         bold: true,
    ///         ..Default::default()
    ///     },
    ///     foreground: Some(Color::Blue),
    ///     ..Default::default()
    /// };
    /// let nu_ansi = style.to_nu_ansi_term_style();
    /// assert_eq!(
    ///     "\u{1b}[0m\u{1b}[01;34mwow\u{1b}[0m",
    ///     nu_ansi.paint("wow").to_string()
    /// );
    /// # }
    /// ```
    ///
    #[cfg(any(feature = "nu-ansi-term", feature = "gnu_legacy"))]
    pub fn to_nu_ansi_term_style(&self) -> nu_ansi_term::Style {
        nu_ansi_term::Style {
            foreground: self.foreground.as_ref().map(Color::to_nu_ansi_term_color),
            background: self.background.as_ref().map(Color::to_nu_ansi_term_color),
            is_bold: self.font_style.bold,
            is_dimmed: self.font_style.dimmed,
            is_italic: self.font_style.italic,
            is_underline: self.font_style.underline,
            is_blink: self.font_style.rapid_blink || self.font_style.slow_blink,
            is_reverse: self.font_style.reverse,
            is_hidden: self.font_style.hidden,
            is_strikethrough: self.font_style.strikethrough,
            #[cfg(feature = "gnu_legacy")]
            prefix_with_reset: true,
            #[cfg(not(feature = "gnu_legacy"))]
            prefix_with_reset: false,
        }
    }

    /// Convert to a `crossterm::style::ContentStyle` (if the `crossterm` feature is enabled).
    #[cfg(feature = "crossterm")]
    pub fn to_crossterm_style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: self.foreground.as_ref().map(Color::to_crossterm_color),
            background_color: self.background.as_ref().map(Color::to_crossterm_color),
            attributes: self.font_style.to_crossterm_attributes(),
            underline_color: self.underline.as_ref().map(Color::to_crossterm_color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_style(
        code: &str,
        foreground: Option<Color>,
        background: Option<Color>,
        underline: Option<Color>,
        font_style: FontStyle,
    ) {
        let style = Style::from_ansi_sequence(code).unwrap();
        assert_eq!(foreground, style.foreground);
        assert_eq!(background, style.background);
        assert_eq!(underline, style.underline);
        assert_eq!(font_style, style.font_style);
    }

    #[test]
    fn parse_simple() {
        assert_style("31", Some(Color::Red), None, None, FontStyle::default());
        assert_style("47", None, Some(Color::White), None, FontStyle::default());
        assert_style(
            "91",
            Some(Color::BrightRed),
            None,
            None,
            FontStyle::default(),
        );
        assert_style(
            "107",
            None,
            Some(Color::BrightWhite),
            None,
            FontStyle::default(),
        );
        assert_style(
            "32;40",
            Some(Color::Green),
            Some(Color::Black),
            None,
            FontStyle::default(),
        );
    }

    #[test]
    fn parse_reject() {
        assert_eq!(None, Style::from_ansi_sequence("a"));
        assert_eq!(None, Style::from_ansi_sequence("1;"));
        assert_eq!(None, Style::from_ansi_sequence("33; 42"));
    }

    #[test]
    fn parse_font_style() {
        assert_style("00;31", Some(Color::Red), None, None, FontStyle::default());
        assert_style("03;34", Some(Color::Blue), None, None, FontStyle::italic());
        assert_style(
            "06;34",
            Some(Color::Blue),
            None,
            None,
            FontStyle::rapid_blink(),
        );
        assert_style("01;36", Some(Color::Cyan), None, None, FontStyle::bold());
        let italic_and_bold = FontStyle {
            bold: true,
            italic: true,
            ..Default::default()
        };
        assert_style("01;03", None, None, None, italic_and_bold);
    }

    #[test]
    fn ignore_unsupported_styles() {
        let style = Style::from_ansi_sequence("14;31").unwrap();
        assert_eq!(Some(Color::Red), style.foreground);
    }

    #[test]
    fn support_reset_of_styles() {
        assert_style("01;31", Some(Color::Red), None, None, FontStyle::bold());
        assert_style(
            "01;31;22",
            Some(Color::Red),
            None,
            None,
            FontStyle::default(),
        );
    }

    #[test]
    fn parse_font_style_backwards() {
        assert_style("34;03", Some(Color::Blue), None, None, FontStyle::italic());
        assert_style("36;01", Some(Color::Cyan), None, None, FontStyle::bold());
        assert_style("31;00", Some(Color::Red), None, None, FontStyle::default());
    }

    #[test]
    fn parse_8_bit_colors() {
        assert_style(
            "38;5;115",
            Some(Color::Fixed(115)),
            None,
            None,
            FontStyle::default(),
        );
        assert_style(
            "00;38;5;115",
            Some(Color::Fixed(115)),
            None,
            None,
            FontStyle::default(),
        );
        assert_style(
            "01;38;5;119",
            Some(Color::Fixed(119)),
            None,
            None,
            FontStyle::bold(),
        );
        assert_style(
            "38;5;119;01",
            Some(Color::Fixed(119)),
            None,
            None,
            FontStyle::bold(),
        );
        assert_style(
            "58;5;115",
            None,
            None,
            Some(Color::Fixed(115)),
            FontStyle::default(),
        );
        assert_style(
            "00;58;5;115",
            None,
            None,
            Some(Color::Fixed(115)),
            FontStyle::default(),
        );
        assert_style(
            "01;58;5;119",
            None,
            None,
            Some(Color::Fixed(119)),
            FontStyle::bold(),
        );
    }

    #[test]
    fn parse_24_bit_colors() {
        assert_style(
            "38;2;115;3;100",
            Some(Color::RGB(115, 3, 100)),
            None,
            None,
            FontStyle::default(),
        );
        assert_style(
            "38;2;115;3;100;3",
            Some(Color::RGB(115, 3, 100)),
            None,
            None,
            FontStyle::italic(),
        );
        assert_style(
            "48;2;100;200;0;1;38;2;0;10;20",
            Some(Color::RGB(0, 10, 20)),
            Some(Color::RGB(100, 200, 0)),
            None,
            FontStyle::bold(),
        );
        assert_style(
            "48;2;100;200;0;1;38;2;0;10;20;58;2;64;64;64",
            Some(Color::RGB(0, 10, 20)),
            Some(Color::RGB(100, 200, 0)),
            Some(Color::RGB(64, 64, 64)),
            FontStyle::bold(),
        );
    }

    #[cfg(all(feature = "nu-ansi-term", not(feature = "gnu_legacy")))]
    #[test]
    fn coloring_nu_ansi_term() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style();
        assert_eq!("\x1b[1;34mwow\x1b[0m", nu_ansi.paint("wow").to_string());
    }

    #[cfg(all(feature = "nu-ansi-term", not(feature = "gnu_legacy")))]
    #[test]
    fn coloring_nu_ansi_term_with_reset() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style().reset_before_style();
        assert_eq!(
            "\x1b[0m\x1b[1;34mwow\x1b[0m",
            nu_ansi.paint("wow").to_string()
        );
    }

    #[cfg(feature = "gnu_legacy")]
    #[test]
    fn coloring_gnu_legacy() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style();
        assert_eq!(
            "\u{1b}[0m\u{1b}[01;34mwow\u{1b}[0m",
            nu_ansi.paint("wow").to_string()
        );
    }

    #[cfg(feature = "gnu_legacy")]
    #[test]
    fn coloring_gnu_legacy_with_reset() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style().reset_before_style();
        assert_eq!(
            "\x1b[0m\x1b[01;34mwow\x1b[0m",
            nu_ansi.paint("wow").to_string()
        );
    }

    #[cfg(feature = "gnu_legacy")]
    #[test]
    fn coloring_gnu_legacy_base() {
        let style = Style {
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style();
        assert_eq!(
            "\u{1b}[0m\u{1b}[mwow\u{1b}[0m",
            nu_ansi.paint("wow").to_string()
        );
    }

    #[cfg(feature = "gnu_legacy")]
    #[test]
    fn coloring_gnu_legacy_base_with_reset() {
        let style = Style {
            ..Default::default()
        };
        let nu_ansi = style.to_nu_ansi_term_style().reset_before_style();
        assert_eq!("\x1b[0m\x1b[mwow\x1b[0m", nu_ansi.paint("wow").to_string());
    }

    #[cfg(feature = "ansi_term")]
    #[test]
    fn coloring_ansi_term() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let ansi = style.to_ansi_term_style();
        assert_eq!("\x1b[1;34mwow\x1b[0m", ansi.paint("wow").to_string());
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn coloring_crossterm() {
        let style = Style {
            font_style: FontStyle {
                bold: true,
                ..Default::default()
            },
            foreground: Some(Color::Blue),
            ..Default::default()
        };
        let cross = style.to_crossterm_style();
        assert_eq!(
            "\x1b[38;5;4m\x1b[1mwow\x1b[0m",
            cross.apply("wow").to_string()
        );
    }
}
