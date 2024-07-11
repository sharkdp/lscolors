//! Suffix styling.
//!
//! GNU ls supports styling regular files based on a suffix (typically a file extension).
//! For example,
//!
//! ```text
//! export LS_COLORS="*.gz=01;31:*.mp3=00;36:"
//! ```
//!
//! will color `archive.tar.gz` bold red, and `music.mp3` cyan.
//!
//! Later suffixes override earlier ones, so
//!
//! ```text
//! export LS_COLORS="*.gz=01;31:*.tar.gz=01;33:"
//! ```
//!
//! will color `foo.gz` and `bar.tar.gz` differently, while
//!
//! ```text
//! export LS_COLORS="*.tar.gz=01;33:*.gz=01;31:"
//! ```
//!
//! will color them both the same.
//!
//! Matching is ASCII case insensitive, unless two different capitalizations with different styles
//! are given for the same suffix:
//!
//! ```text
//! export LS_COLORS="*README=01:*readme=00:"
//! ```

use aho_corasick::{AhoCorasick, Anchored, Input, MatchKind, StartKind};

use std::collections::{HashMap, HashSet};

use crate::style::Style;

/// A key in the suffix map.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct SuffixKey {
    /// The suffix, reversed so we can use leftmost-first matching.
    rev_bytes: Box<[u8]>,
}

impl SuffixKey {
    fn new(suffix: &[u8]) -> Self {
        let mut suffix: Box<[u8]> = Box::from(suffix);
        suffix.reverse();
        Self { rev_bytes: suffix }
    }
}

impl AsRef<[u8]> for SuffixKey {
    fn as_ref(&self) -> &[u8] {
        &self.rev_bytes
    }
}

/// A [SuffixMap] builder.
#[derive(Debug, Default)]
pub struct SuffixMapBuilder {
    /// The list of keys, in order.
    keys: Vec<SuffixKey>,
    /// The list of styles, in order.
    styles: Vec<Option<Style>>,
    /// The length of the longest suffix, in bytes.
    max_len: usize,
}

impl SuffixMapBuilder {
    /// Add a new suffix to the map.
    pub fn push(&mut self, suffix: impl AsRef<[u8]>, style: Option<Style>) {
        let suffix = suffix.as_ref();
        self.keys.push(SuffixKey::new(suffix));
        self.styles.push(style);
        self.max_len = self.max_len.max(suffix.len());
    }

    /// Build the suffix map.
    pub fn build(mut self) -> SuffixMap {
        // Reverse the lists, so that leftmost-*first* returns the *last* match instead
        self.keys.reverse();
        self.styles.reverse();

        // Build the case-sensitive matcher
        let cs_matcher = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostFirst)
            .start_kind(StartKind::Anchored)
            .build(&self.keys)
            .unwrap();

        // Turn all the keys lowercase
        let mut lower_keys = self.keys.clone();
        for key in lower_keys.iter_mut() {
            key.rev_bytes.make_ascii_lowercase();
        }

        // Map keys to their first case-(in)sensitive occurrence
        let mut cs_map: HashMap<&SuffixKey, usize> = HashMap::new();
        let mut ci_map: HashMap<&SuffixKey, usize> = HashMap::new();
        self.keys
            .iter()
            .zip(lower_keys.iter())
            .enumerate()
            .for_each(|(i, (cs_key, ci_key))| {
                cs_map.entry(cs_key).or_insert(i);
                ci_map.entry(ci_key).or_insert(i);
            });

        // Find keys that should be case-sensitive
        let mut cs_set: HashSet<&SuffixKey> = HashSet::new();
        for i in cs_map.values().copied() {
            let ci_key = &lower_keys[i];
            let j = *ci_map.get(ci_key).unwrap();
            if self.styles[i] != self.styles[j] {
                cs_set.insert(ci_key);
            }
        }

        // Keep only the case-insensitive keys
        let (ci_ids, ci_keys): (Vec<_>, Vec<_>) = lower_keys
            .iter()
            .enumerate()
            .filter(|(_i, k)| !cs_set.contains(k))
            .unzip();

        // Build the case-insensitive matcher
        let ci_matcher = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .match_kind(MatchKind::LeftmostFirst)
            .start_kind(StartKind::Anchored)
            .build(ci_keys)
            .unwrap();

        SuffixMap {
            cs_matcher,
            ci_matcher,
            styles: self.styles,
            ci_ids,
            max_len: self.max_len,
        }
    }
}

/// Maps filename suffixes to styles.
#[derive(Clone, Debug)]
pub struct SuffixMap {
    /// Case-sensitive matcher.
    cs_matcher: AhoCorasick,
    /// Case-insensitive suffixes.
    ci_matcher: AhoCorasick,
    /// List of styles (indexed by cs_matcher IDs)
    styles: Vec<Option<Style>>,
    /// Map from ci_matcher to cs_matcher IDs.
    ci_ids: Vec<usize>,
    /// The length of the longest suffix, in bytes.
    max_len: usize,
}

impl SuffixMap {
    /// Get the style for a matching suffix, if one exists.
    pub fn get(&self, name: impl AsRef<[u8]>) -> Option<&Style> {
        let name = name.as_ref();

        // Split off only the longest suffix necessary
        let len = self.max_len.min(name.len());
        let i = name.len() - len;

        // Copy the suffix to the stack if small, otherwise the heap
        let mut name_stack = [0; 32];
        let mut name_heap: Box<[u8]>;

        let name = if len <= name_stack.len() {
            name_stack[..len].copy_from_slice(&name[i..]);
            &mut name_stack[..len]
        } else {
            name_heap = name[i..].into();
            &mut name_heap
        };

        // Reverse the suffix for matching
        name.reverse();

        // Find a case-sensitive match
        let cs_index = Self::find(&self.cs_matcher, &name);

        // Find a case-insensitive match
        let ci_index = Self::find(&self.ci_matcher, &name).map(|i| self.ci_ids[i]);

        // Return the later match (earlier index)
        let i = match (cs_index, ci_index) {
            (Some(cs), Some(ci)) => cs.min(ci),
            (Some(cs), _) => cs,
            (_, Some(ci)) => ci,
            (_, _) => return None,
        };
        self.styles[i].as_ref()
    }

    /// Get the index of a match in a single matcher.
    fn find(matcher: &AhoCorasick, name: &[u8]) -> Option<usize> {
        let input = Input::new(name).anchored(Anchored::Yes);
        matcher.find(input).map(|m| m.pattern().as_usize())
    }
}
