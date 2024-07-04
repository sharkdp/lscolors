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
//! // If you want to use `nu_ansi_term`:
//! # #[cfg(features = "nu-ansi-term")]
//! # {
//! let ansi_style = style.map(Style::to_nu_ansi_term_style).unwrap_or_default();
//! println!("{}", ansi_style.paint(path));
//! # }
//!
//! // If you want to use `ansi_term`:
//! # #[cfg(features = "ansi_term")]
//! # {
//! let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
//! println!("{}", ansi_style.paint(path));
//! # }
//! ```

mod fs;
pub mod style;
mod suffix;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata};
use std::path::{Component, Path, PathBuf, MAIN_SEPARATOR};

use crate::suffix::{SuffixMap, SuffixMapBuilder};

pub use crate::style::{Color, FontStyle, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Indicator {
    /// `no`: Normal (non-filename) text
    Normal,

    /// `fi`: Regular file
    RegularFile,

    /// `di`: Directory
    Directory,

    /// `ln`: Symbolic link
    SymbolicLink,

    /// `pi`: Named pipe or FIFO
    FIFO,

    /// `so`: Socket
    Socket,

    /// `do`: Door (IPC connection to another program)
    Door,

    /// `bd`: Block-oriented device
    BlockDevice,

    /// `cd`: Character-oriented device
    CharacterDevice,

    /// `or`: A broken symbolic link
    OrphanedSymbolicLink,

    /// `su`: A file that is setuid (`u+s`)
    Setuid,

    /// `sg`: A file that is setgid (`g+s`)
    Setgid,

    /// `st`: A directory that is sticky and other-writable (`+t`, `o+w`)
    Sticky,

    /// `ow`: A directory that is not sticky and other-writeable (`o+w`)
    OtherWritable,

    /// `tw`: A directory that is sticky and other-writable (`+t`, `o+w`)
    StickyAndOtherWritable,

    /// `ex`: Executable file
    ExecutableFile,

    /// `mi`: Missing file
    MissingFile,

    /// `ca`: File with capabilities set
    Capabilities,

    /// `mh`: File with multiple hard links
    MultipleHardLinks,

    /// `lc`: Code that is printed before the color sequence
    LeftCode,

    /// `rc`: Code that is printed after the color sequence
    RightCode,

    /// `ec`: End code
    EndCode,

    /// `rs`: Code to reset to ordinary colors
    Reset,

    /// `cl`: Code to clear to the end of the line
    ClearLine,
}

impl Indicator {
    pub fn from(indicator: &str) -> Option<Indicator> {
        match indicator {
            "no" => Some(Indicator::Normal),
            "fi" => Some(Indicator::RegularFile),
            "di" => Some(Indicator::Directory),
            "ln" => Some(Indicator::SymbolicLink),
            "pi" => Some(Indicator::FIFO),
            "so" => Some(Indicator::Socket),
            "do" => Some(Indicator::Door),
            "bd" => Some(Indicator::BlockDevice),
            "cd" => Some(Indicator::CharacterDevice),
            "or" => Some(Indicator::OrphanedSymbolicLink),
            "su" => Some(Indicator::Setuid),
            "sg" => Some(Indicator::Setgid),
            "st" => Some(Indicator::Sticky),
            "ow" => Some(Indicator::OtherWritable),
            "tw" => Some(Indicator::StickyAndOtherWritable),
            "ex" => Some(Indicator::ExecutableFile),
            "mi" => Some(Indicator::MissingFile),
            "ca" => Some(Indicator::Capabilities),
            "mh" => Some(Indicator::MultipleHardLinks),
            "lc" => Some(Indicator::LeftCode),
            "rc" => Some(Indicator::RightCode),
            "ec" => Some(Indicator::EndCode),
            "rs" => Some(Indicator::Reset),
            "cl" => Some(Indicator::ClearLine),
            _ => None,
        }
    }
}

/// Iterator over the path components with their respective style.
pub struct StyledComponents<'a> {
    /// Reference to the underlying LsColors object
    lscolors: &'a LsColors,

    /// Full path to the current component
    component_path: PathBuf,

    /// Underlying iterator over the path components
    components: std::iter::Peekable<std::path::Components<'a>>,
}

impl<'a> Iterator for StyledComponents<'a> {
    type Item = (OsString, Option<&'a Style>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(component) = self.components.next() {
            let mut component_str = component.as_os_str().to_os_string();

            self.component_path.push(&component_str);
            let style = self.lscolors.style_for_path(&self.component_path);

            if self.components.peek().is_some() {
                match component {
                    // Prefix needs no separator, as it is always followed by RootDir.
                    // RootDir is already a separator.
                    Component::Prefix(_) | Component::RootDir => {}
                    // Everything else uses a separator that is painted the same way as the component.
                    Component::CurDir | Component::ParentDir | Component::Normal(_) => {
                        component_str.push(MAIN_SEPARATOR.to_string());
                    }
                }
            }

            Some((component_str, style))
        } else {
            None
        }
    }
}

/// A colorable file path.
pub trait Colorable {
    /// Get the full path to this file.
    fn path(&self) -> PathBuf;

    /// Get the name of this file.
    fn file_name(&self) -> OsString;

    /// Try to get the type of this file.
    fn file_type(&self) -> Option<FileType>;

    /// Try to get the metadata for this file.
    fn metadata(&self) -> Option<Metadata>;
}

impl Colorable for DirEntry {
    fn path(&self) -> PathBuf {
        self.path()
    }

    fn file_name(&self) -> OsString {
        self.file_name()
    }

    fn file_type(&self) -> Option<FileType> {
        self.file_type().ok()
    }

    fn metadata(&self) -> Option<Metadata> {
        self.metadata().ok()
    }
}

/// Builder for [LsColors].
struct LsColorsBuilder {
    indicator_mapping: HashMap<Indicator, Style>,

    /// Whether Indicator::RegularFile falls back to Indicator::Normal
    /// (see <https://github.com/sharkdp/lscolors/issues/48#issuecomment-1582830387>)
    file_normal_fallback: bool,

    suffixes: SuffixMapBuilder,
}

impl LsColorsBuilder {
    fn empty() -> Self {
        Self {
            indicator_mapping: HashMap::new(),
            file_normal_fallback: true,
            suffixes: SuffixMapBuilder::default(),
        }
    }

    fn add_from_string(&mut self, input: &str) {
        for entry in input.split(':') {
            let parts: Vec<_> = entry.split('=').collect();

            if let Some([entry, ansi_style]) = parts.get(0..2) {
                let style = Style::from_ansi_sequence(ansi_style);
                if let Some(suffix) = entry.strip_prefix('*') {
                    self.suffixes.push(suffix, style);
                } else if let Some(indicator) = Indicator::from(entry) {
                    if let Some(style) = style {
                        self.indicator_mapping.insert(indicator, style);
                    } else {
                        self.indicator_mapping.remove(&indicator);
                        if indicator == Indicator::RegularFile {
                            self.file_normal_fallback = false;
                        }
                    }
                }
            }
        }
    }

    fn build(self) -> LsColors {
        LsColors {
            indicator_mapping: self.indicator_mapping,
            file_normal_fallback: self.file_normal_fallback,
            suffixes: self.suffixes.build(),
        }
    }
}

const LS_COLORS_DEFAULT: &str = "rs=0:lc=\x1b[:rc=m:cl=\x1b[K:ex=01;32:sg=30;43:su=37;41:di=01;34:st=37;44:ow=34;42:tw=30;42:ln=01;36:bd=01;33:cd=01;33:do=01;35:pi=33:so=01;35:";

impl Default for LsColorsBuilder {
    fn default() -> Self {
        let mut builder = Self::empty();
        builder.add_from_string(LS_COLORS_DEFAULT);
        builder
    }
}

/// Holds information about how different file system entries should be colorized / styled.
#[derive(Debug, Clone)]
pub struct LsColors {
    indicator_mapping: HashMap<Indicator, Style>,

    /// Whether Indicator::RegularFile falls back to Indicator::Normal
    /// (see <https://github.com/sharkdp/lscolors/issues/48#issuecomment-1582830387>)
    file_normal_fallback: bool,

    suffixes: SuffixMap,
}

impl Default for LsColors {
    /// Constructs a default `LsColors` instance with some default styles. See `man dircolors` for
    /// information about the default styles and colors.
    fn default() -> Self {
        LsColorsBuilder::default().build()
    }
}

impl LsColors {
    /// Construct an empty [`LsColors`](struct.LsColors.html) instance with no pre-defined styles.
    pub fn empty() -> Self {
        LsColorsBuilder::empty().build()
    }

    /// Creates a new [`LsColors`](struct.LsColors.html) instance from the `LS_COLORS` environment
    /// variable. The basis for this is a default style as constructed via the `Default`
    /// implementation.
    pub fn from_env() -> Option<Self> {
        env::var("LS_COLORS")
            .ok()
            .as_ref()
            .map(|s| Self::from_string(s))
    }

    /// Creates a new [`LsColors`](struct.LsColors.html) instance from the given string.
    pub fn from_string(input: &str) -> Self {
        let mut builder = LsColorsBuilder::default();
        builder.add_from_string(input);
        builder.build()
    }

    /// Get the ANSI style for a given path.
    ///
    /// *Note:* this function calls `Path::symlink_metadata` internally. If you already happen to
    /// have the `Metadata` available, use [`style_for_path_with_metadata`](#method.style_for_path_with_metadata).
    pub fn style_for_path<P: AsRef<Path>>(&self, path: P) -> Option<&Style> {
        let metadata = path.as_ref().symlink_metadata().ok();
        self.style_for_path_with_metadata(path, metadata.as_ref())
    }

    /// Check if an indicator has an associated color.
    fn has_color_for(&self, indicator: Indicator) -> bool {
        self.indicator_mapping.contains_key(&indicator)
    }

    /// Check if we need metadata to color a regular file.
    fn needs_file_metadata(&self) -> bool {
        self.has_color_for(Indicator::Setuid)
            || self.has_color_for(Indicator::Setgid)
            || self.has_color_for(Indicator::ExecutableFile)
            || self.has_color_for(Indicator::MultipleHardLinks)
    }

    /// Check if we need metadata to color a directory.
    fn needs_dir_metadata(&self) -> bool {
        self.has_color_for(Indicator::StickyAndOtherWritable)
            || self.has_color_for(Indicator::OtherWritable)
            || self.has_color_for(Indicator::Sticky)
    }

    /// Get the indicator type for a path with corresponding metadata.
    fn indicator_for<F: Colorable>(&self, file: &F) -> Indicator {
        let file_type = file.file_type();

        if let Some(file_type) = file_type {
            if file_type.is_file() {
                if self.needs_file_metadata() {
                    if let Some(metadata) = file.metadata() {
                        let mode = crate::fs::mode(&metadata);
                        let nlink = crate::fs::nlink(&metadata);

                        if self.has_color_for(Indicator::Setuid) && mode & 0o4000 != 0 {
                            return Indicator::Setuid;
                        } else if self.has_color_for(Indicator::Setgid) && mode & 0o2000 != 0 {
                            return Indicator::Setgid;
                        } else if self.has_color_for(Indicator::ExecutableFile)
                            && mode & 0o0111 != 0
                        {
                            return Indicator::ExecutableFile;
                        } else if self.has_color_for(Indicator::MultipleHardLinks) && nlink > 1 {
                            return Indicator::MultipleHardLinks;
                        }
                    }
                }

                Indicator::RegularFile
            } else if file_type.is_dir() {
                if self.needs_dir_metadata() {
                    if let Some(metadata) = file.metadata() {
                        let mode = crate::fs::mode(&metadata);

                        if self.has_color_for(Indicator::StickyAndOtherWritable)
                            && mode & 0o1002 == 0o1002
                        {
                            return Indicator::StickyAndOtherWritable;
                        } else if self.has_color_for(Indicator::OtherWritable) && mode & 0o0002 != 0
                        {
                            return Indicator::OtherWritable;
                        } else if self.has_color_for(Indicator::Sticky) && mode & 0o1000 != 0 {
                            return Indicator::Sticky;
                        }
                    }
                }

                Indicator::Directory
            } else if file_type.is_symlink() {
                // This works because `Path::exists` traverses symlinks.
                if self.has_color_for(Indicator::OrphanedSymbolicLink) && !file.path().exists() {
                    return Indicator::OrphanedSymbolicLink;
                }

                Indicator::SymbolicLink
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::FileTypeExt;

                    if file_type.is_fifo() {
                        return Indicator::FIFO;
                    }
                    if file_type.is_socket() {
                        return Indicator::Socket;
                    }
                    if file_type.is_block_device() {
                        return Indicator::BlockDevice;
                    }
                    if file_type.is_char_device() {
                        return Indicator::CharacterDevice;
                    }
                }

                // Treat files of unknown type as errors
                Indicator::MissingFile
            }
        } else {
            // Default to a regular file, so we still try the suffix map when no metadata is available
            Indicator::RegularFile
        }
    }

    /// Get the ANSI style for a colorable path.
    pub fn style_for<F: Colorable>(&self, file: &F) -> Option<&Style> {
        let indicator = self.indicator_for(file);

        if indicator == Indicator::RegularFile {
            // Note: using '.to_str()' here means that filename
            // matching will not work with invalid-UTF-8 paths.
            let filename = file.file_name();
            if let Some(style) = self.style_for_str(filename.to_str()?) {
                return Some(style);
            }
        }

        self.style_for_indicator(indicator)
    }

    /// Get the ANSI style for a string. This does not have to be a valid filepath.
    pub fn style_for_str(&self, file_str: &str) -> Option<&Style> {
        self.suffixes.get(file_str)
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
        struct PathWithMetadata<'a> {
            path: &'a Path,
            metadata: Option<&'a Metadata>,
        }

        impl Colorable for PathWithMetadata<'_> {
            fn path(&self) -> PathBuf {
                self.path.to_owned()
            }

            fn file_name(&self) -> OsString {
                // Path::file_name() only works if the last component is Normal, but
                // we want it for all component types, so we open code it

                self.path
                    .components()
                    .last()
                    .map(|c| c.as_os_str())
                    .unwrap_or_else(|| self.path.as_os_str())
                    .to_owned()
            }

            fn file_type(&self) -> Option<FileType> {
                self.metadata.map(|m| m.file_type())
            }

            fn metadata(&self) -> Option<Metadata> {
                self.metadata.cloned()
            }
        }

        let path = path.as_ref();
        self.style_for(&PathWithMetadata { path, metadata })
    }

    /// Get ANSI styles for each component of a given path. Components already include the path
    /// separator symbol, if required. For a path like `foo/bar/test.md`, this would return an
    /// iterator over three pairs for the three path components `foo/`, `bar/` and `test.md`
    /// together with their respective styles.
    pub fn style_for_path_components<'a>(&'a self, path: &'a Path) -> StyledComponents<'a> {
        StyledComponents {
            lscolors: self,
            component_path: PathBuf::new(),
            components: path.components().peekable(),
        }
    }

    /// Get the ANSI style for a certain `Indicator` (regular file, directory, symlink, ...). Note
    /// that this function implements a fallback logic for some of the indicators (just like `ls`).
    /// For example, the style for `mi` (missing file) falls back to `or` (orphaned symbolic link)
    /// if it has not been specified explicitly.
    pub fn style_for_indicator(&self, indicator: Indicator) -> Option<&Style> {
        self.indicator_mapping
            .get(&indicator)
            .or_else(|| {
                self.indicator_mapping.get(&match indicator {
                    Indicator::Setuid
                    | Indicator::Setgid
                    | Indicator::ExecutableFile
                    | Indicator::MultipleHardLinks => Indicator::RegularFile,

                    Indicator::StickyAndOtherWritable
                    | Indicator::OtherWritable
                    | Indicator::Sticky => Indicator::Directory,

                    Indicator::OrphanedSymbolicLink => Indicator::SymbolicLink,

                    Indicator::MissingFile => Indicator::OrphanedSymbolicLink,

                    _ => indicator,
                })
            })
            .or_else(|| {
                if indicator == Indicator::RegularFile && !self.file_normal_fallback {
                    None
                } else {
                    self.indicator_mapping.get(&Indicator::Normal)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, FontStyle, Style};
    use crate::{Indicator, LsColors};

    use std::fs::{self, File};
    use std::path::{Path, PathBuf};

    #[test]
    fn basic_usage() {
        let lscolors = LsColors::from_string("*.wav=00;36:");

        let style_dir = lscolors.style_for_indicator(Indicator::Directory).unwrap();
        assert_eq!(FontStyle::bold(), style_dir.font_style);
        assert_eq!(Some(Color::Blue), style_dir.foreground);
        assert_eq!(None, style_dir.background);

        let style_symlink = lscolors
            .style_for_indicator(Indicator::SymbolicLink)
            .unwrap();
        assert_eq!(FontStyle::bold(), style_symlink.font_style);
        assert_eq!(Some(Color::Cyan), style_symlink.foreground);
        assert_eq!(None, style_symlink.background);

        let style_rs = lscolors.style_for_path("test.wav").unwrap();
        assert_eq!(FontStyle::default(), style_rs.font_style);
        assert_eq!(Some(Color::Cyan), style_rs.foreground);
        assert_eq!(None, style_rs.background);
    }

    #[test]
    fn style_for_path_uses_correct_ordering() {
        let lscolors = LsColors::from_string("*.foo=01;35:*README.foo=33;44");

        // dummy.foo matches to *.foo without getting overriden.
        let style_foo = lscolors.style_for_path("some/folder/dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);
        assert_eq!(None, style_foo.background);

        // README.foo matches to *README.foo by overriding *.foo
        let style_readme = lscolors
            .style_for_path("some/other/folder/README.foo")
            .unwrap();
        assert_eq!(FontStyle::default(), style_readme.font_style);
        assert_eq!(Some(Color::Yellow), style_readme.foreground);
        assert_eq!(Some(Color::Blue), style_readme.background);

        let lscolors = LsColors::from_string("*README.foo=33;44:*.foo=01;35");

        let style_foo = lscolors.style_for_path("some/folder/dummy.foo").unwrap();
        assert_eq!(FontStyle::bold(), style_foo.font_style);
        assert_eq!(Some(Color::Magenta), style_foo.foreground);
        assert_eq!(None, style_foo.background);

        // README.foo matches to *.foo because *.foo overrides *README.foo
        let style_readme = lscolors
            .style_for_path("some/other/folder/README.foo")
            .unwrap();
        assert_eq!(FontStyle::bold(), style_readme.font_style);
        assert_eq!(Some(Color::Magenta), style_readme.foreground);
        assert_eq!(None, style_readme.background);
    }

    #[test]
    fn style_for_path_uses_lowercase_matching() {
        let lscolors = LsColors::from_string("*.O=01;35");

        let style_artifact = lscolors.style_for_path("artifact.o").unwrap();
        assert_eq!(FontStyle::bold(), style_artifact.font_style);
        assert_eq!(Some(Color::Magenta), style_artifact.foreground);
        assert_eq!(None, style_artifact.background);
    }

    #[test]
    fn default_styles_should_be_preserved() {
        // Setting an unrelated style should not influence the default
        // style for "directory" (below)
        let lscolors = LsColors::from_string("ex=01:");

        let style_dir = lscolors.style_for_indicator(Indicator::Directory).unwrap();
        assert_eq!(FontStyle::bold(), style_dir.font_style);
        assert_eq!(Some(Color::Blue), style_dir.foreground);
        assert_eq!(None, style_dir.background);
    }

    fn temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("temporary directory")
    }

    fn create_file<P: AsRef<Path>>(path: P) -> PathBuf {
        File::create(&path).expect("temporary file");
        path.as_ref().to_path_buf()
    }

    fn create_dir<P: AsRef<Path>>(path: P) -> PathBuf {
        fs::create_dir(&path).expect("temporary directory");
        path.as_ref().to_path_buf()
    }

    fn get_default_style<P: AsRef<Path>>(path: P) -> Option<Style> {
        let lscolors = LsColors::default();
        lscolors.style_for_path(path).cloned()
    }

    #[cfg(unix)]
    fn create_symlink<P: AsRef<Path>>(from: P, to: P) {
        std::os::unix::fs::symlink(from, to).expect("temporary symlink");
    }

    #[cfg(windows)]
    fn create_symlink<P: AsRef<Path>>(src: P, dst: P) {
        if src.as_ref().is_dir() {
            std::os::windows::fs::symlink_dir(src, dst).expect("temporary symlink");
        } else {
            std::os::windows::fs::symlink_file(src, dst).expect("temporary symlink");
        }
    }

    #[test]
    fn style_for_str() {
        let lscolors = LsColors::from_string("*.wav=00;36:*.rs=1;38;5;202:");

        assert_eq!(lscolors.style_for_str(""), None);
        assert_eq!(lscolors.style_for_str("test"), None);
        assert_eq!(
            lscolors.style_for_str("test.wav").unwrap().foreground,
            Some(Color::Cyan)
        );
        assert_eq!(
            lscolors.style_for_str("test.rs").unwrap().foreground,
            Some(Color::Fixed(202))
        );
    }

    #[test]
    fn style_for_directory() {
        let tmp_dir = temp_dir();
        let style = get_default_style(tmp_dir.path()).unwrap();
        assert_eq!(Some(Color::Blue), style.foreground);
    }

    #[test]
    fn style_for_file() {
        let tmp_dir = temp_dir();
        let tmp_file_path = create_file(tmp_dir.path().join("test-file"));
        let style = get_default_style(tmp_file_path);
        assert_eq!(None, style);
    }

    #[test]
    fn style_for_symlink() {
        let tmp_dir = temp_dir();
        let tmp_file_path = create_file(tmp_dir.path().join("test-file"));
        let tmp_symlink_path = tmp_dir.path().join("test-symlink");

        create_symlink(&tmp_file_path, &tmp_symlink_path);

        let style = get_default_style(tmp_symlink_path).unwrap();
        assert_eq!(Some(Color::Cyan), style.foreground);
    }

    #[test]
    fn style_for_broken_symlink() {
        let tmp_dir = temp_dir();
        let tmp_file_path = tmp_dir.path().join("non-existing-file");
        let tmp_symlink_path = tmp_dir.path().join("broken-symlink");

        create_symlink(&tmp_file_path, &tmp_symlink_path);

        let lscolors = LsColors::from_string("or=40;31;01:");
        let style = lscolors.style_for_path(tmp_symlink_path).unwrap();
        assert_eq!(Some(Color::Red), style.foreground);
    }

    #[test]
    fn style_for_missing_file() {
        let lscolors1 = LsColors::from_string("mi=01:or=33;44");

        let style_missing = lscolors1
            .style_for_indicator(Indicator::MissingFile)
            .unwrap();
        assert_eq!(FontStyle::bold(), style_missing.font_style);

        let lscolors2 = LsColors::from_string("or=33;44");

        let style_missing = lscolors2
            .style_for_indicator(Indicator::MissingFile)
            .unwrap();
        assert_eq!(Some(Color::Yellow), style_missing.foreground);

        let lscolors3 = LsColors::from_string("or=33;44:mi=00");

        let style_missing = lscolors3
            .style_for_indicator(Indicator::MissingFile)
            .unwrap();
        assert_eq!(Some(Color::Yellow), style_missing.foreground);
    }

    #[cfg(unix)]
    #[test]
    fn style_for_setid() {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;

        let tmp_dir = temp_dir();
        let tmp_file = create_file(tmp_dir.path().join("setid"));
        let perms = Permissions::from_mode(0o6750);
        set_permissions(&tmp_file, perms).unwrap();

        let suid_style = get_default_style(&tmp_file).unwrap();
        assert_eq!(Some(Color::Red), suid_style.background);

        let lscolors = LsColors::from_string("su=0");
        let sgid_style = lscolors.style_for_path(&tmp_file).unwrap();
        assert_eq!(Some(Color::Yellow), sgid_style.background);
    }

    #[cfg(unix)]
    #[test]
    fn style_for_multi_hard_links() {
        let tmp_dir = temp_dir();
        let tmp_file = create_file(tmp_dir.path().join("file1"));
        std::fs::hard_link(&tmp_file, tmp_dir.path().join("file2")).unwrap();

        let lscolors = LsColors::from_string("mh=35");
        let style = lscolors.style_for_path(&tmp_file).unwrap();
        assert_eq!(Some(Color::Magenta), style.foreground);
    }

    #[cfg(unix)]
    #[test]
    fn style_for_sticky_other_writable() {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;

        let tmp_root = temp_dir();
        let tmp_dir = create_dir(tmp_root.path().join("test-dir"));
        let perms = Permissions::from_mode(0o1777);
        set_permissions(&tmp_dir, perms).unwrap();

        let so_style = get_default_style(&tmp_dir).unwrap();
        assert_eq!(Some(Color::Black), so_style.foreground);
        assert_eq!(Some(Color::Green), so_style.background);

        let lscolors1 = LsColors::from_string("tw=0");
        let ow_style = lscolors1.style_for_path(&tmp_dir).unwrap();
        assert_eq!(Some(Color::Blue), ow_style.foreground);
        assert_eq!(Some(Color::Green), ow_style.background);

        let lscolors2 = LsColors::from_string("tw=0:ow=0");
        let st_style = lscolors2.style_for_path(&tmp_dir).unwrap();
        assert_eq!(Some(Color::White), st_style.foreground);
        assert_eq!(Some(Color::Blue), st_style.background);
    }

    #[test]
    fn style_for_path_components() {
        use std::ffi::OsString;

        let tmp_root = temp_dir();
        let tmp_dir = create_dir(tmp_root.path().join("test-dir"));
        create_file(tmp_root.path().join("test-file.png"));

        let tmp_symlink = tmp_root.path().join("test-symlink");
        create_symlink(&tmp_dir, &tmp_symlink);

        let path_via_symlink = tmp_symlink.join("test-file.png");

        let lscolors = LsColors::from_string("di=34:ln=35:*.png=36");

        let mut components: Vec<_> = lscolors
            .style_for_path_components(&path_via_symlink)
            .collect();

        let (c_file, style_file) = components.pop().unwrap();
        assert_eq!("test-file.png", c_file);
        assert_eq!(Some(Color::Cyan), style_file.unwrap().foreground);

        let (c_symlink, style_symlink) = components.pop().unwrap();
        let mut expected_symlink_name = OsString::from("test-symlink");
        expected_symlink_name.push(std::path::MAIN_SEPARATOR.to_string());
        assert_eq!(expected_symlink_name, c_symlink);
        assert_eq!(
            Some(Color::Magenta),
            style_symlink.cloned().and_then(|style| style.foreground)
        );

        let (_, style_dir) = components.pop().unwrap();
        assert_eq!(Some(Color::Blue), style_dir.unwrap().foreground);
    }

    #[test]
    fn style_for_dir_entry() {
        use std::fs::read_dir;

        let tmp_root = temp_dir();
        create_file(tmp_root.path().join("test-file.png"));

        let lscolors = LsColors::from_string("*.png=01;35");

        for entry in read_dir(tmp_root.path()).unwrap() {
            let style = lscolors.style_for(&entry.unwrap()).unwrap();
            assert_eq!(Some(Color::Magenta), style.foreground);
        }
    }

    #[test]
    fn override_disable_suffix() {
        let tmp_dir = temp_dir();
        let tmp_file = create_file(tmp_dir.path().join("test-file.png"));

        let lscolors = LsColors::from_string("*.png=01;35:*.png=0");
        let style = lscolors.style_for_path(&tmp_file);
        assert_eq!(None, style);
    }

    #[test]
    fn file_normal_fallback() {
        let tmp_dir = temp_dir();
        let tmp_file_path = create_file(tmp_dir.path().join("test-file"));

        let lscolors = LsColors::from_string("no=01;31");
        let style = lscolors.style_for_path(&tmp_file_path).unwrap();
        assert_eq!(Some(Color::Red), style.foreground);

        let lscolors = LsColors::from_string("no=01;31:fi=0");
        let style = lscolors.style_for_path(&tmp_file_path);
        assert_eq!(None, style);
    }

    #[test]
    fn file_suffix_case() {
        let assert_bold_fg_magenta = |style: Option<&Style>| {
            Some(Color::Magenta) == style.and_then(|v| v.foreground)
                && matches!(style,Some(sty) if sty.font_style.bold)
        };
        let assert_bold_fg_green = |style: Option<&Style>| {
            Some(Color::Green) == style.and_then(|v| v.foreground)
                && matches!(style,Some(sty) if sty.font_style.bold)
        };

        // *.jpg is specified only once so any suffix that has .jpg should match
        // without caring about the letter case
        let lscolors = LsColors::from_string("*.jpg=01;35:*.Z=01;31");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_magenta(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_magenta(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(assert_bold_fg_magenta(mixedcase_suffix));

        // *.jpg is specified more than once with different cases and style, so
        // case should matter here
        let lscolors = LsColors::from_string("*.jpg=01;35:*.JPG=01;32");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_magenta(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_green(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(mixedcase_suffix.is_none());

        // *.jpg is specified more than once with different cases but style is same, so
        // case can ignored
        let lscolors = LsColors::from_string("*.jpg=01;35:*.JPG=01;35");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_magenta(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_magenta(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(assert_bold_fg_magenta(mixedcase_suffix));

        // last *.jpg gets more priority resulting in same style across
        // different cases specified, so case can ignored
        let lscolors = LsColors::from_string("*.jpg=01;32:*.jpg=01;35:*.JPG=01;35");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_magenta(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_magenta(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(assert_bold_fg_magenta(mixedcase_suffix));

        // same as above with different combinations
        let lscolors = LsColors::from_string("*.jpg=01;32:*.jpg=01;35:*.JPG=01;32:*.jpg=01;32");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_green(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_green(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(assert_bold_fg_green(mixedcase_suffix));

        // last *.jpg gets more priority resulting in different style across
        // different cases specified, so case matters
        let lscolors = LsColors::from_string("*.jpg=01;32:*.jpg=01;35:*.JPG=01;32");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_magenta(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_green(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(mixedcase_suffix.is_none());

        // same as above with different combinations
        let lscolors = LsColors::from_string("*.jpg=01;32:*.jpg=01;35:*.JPG=01;35:*.jpg=01;32");
        let lowercase_suffix = lscolors.style_for_str("img1.jpg");
        assert!(assert_bold_fg_green(lowercase_suffix));
        let uppercase_suffix = lscolors.style_for_str("img1.JPG");
        assert!(assert_bold_fg_magenta(uppercase_suffix));
        let mixedcase_suffix = lscolors.style_for_str("img1.JpG");
        assert!(mixedcase_suffix.is_none());
    }
}
