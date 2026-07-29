pub mod comparator;

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::num::NonZeroUsize;

/// Extracts the Nth (1-indexed) field from a line.
/// When `sep` is `None`, splits on whitespace. When `sep` is `Some(c)`, splits on that character.
/// Returns an empty string if the line has fewer than `n` fields.
pub fn extract_key_field(line: &str, n: NonZeroUsize, sep: Option<char>) -> &str {
    let idx = n.get() - 1;
    match sep {
        None => line.split_whitespace().nth(idx).unwrap_or(""),
        Some(c) => line.split(c).nth(idx).unwrap_or(""),
    }
}

/// A sorted line paired with its optional pre-extracted sort key (`-k`).
struct Entry {
    key: Option<String>,
    line: String,
}

impl Entry {
    fn key(&self) -> &str {
        self.key.as_deref().unwrap_or(&self.line)
    }
}

/// The full sorting policy: comparator, direction, dedup, output window,
/// and `-k` key extraction.
pub struct SortPolicy {
    pub cmp_fn: fn(&str, &str) -> Ordering,
    pub reverse: bool,
    pub unique: bool,
    pub top: Option<usize>,
    pub bottom: Option<usize>,
    pub key_field: Option<NonZeroUsize>,
    pub field_sep: Option<char>,
}

/// Maintains lines in sorted order under a fixed [`SortPolicy`],
/// one [`insert`](Self::insert) at a time.
pub struct SortedBuffer {
    entries: VecDeque<Entry>,
    policy: SortPolicy,
}

impl SortedBuffer {
    #[must_use]
    pub fn new(policy: SortPolicy) -> Self {
        Self {
            entries: VecDeque::new(),
            policy,
        }
    }

    /// Inserts `line` (without its trailing newline) at its sorted position.
    /// Returns `false` if the line was skipped: a duplicate under `unique`,
    /// or a line falling outside the `top`/`bottom` window.
    pub fn insert(&mut self, line: &str) -> bool {
        let cached_key = self
            .policy
            .key_field
            .map(|n| extract_key_field(line, n, self.policy.field_sep));

        let search_result = self.entries.binary_search_by(|e| {
            let key_line = cached_key.unwrap_or(line);
            let ord = match (self.policy.cmp_fn)(e.key(), key_line) {
                Ordering::Equal => comparator::compare_normal(&e.line, line),
                other => other,
            };
            if self.policy.reverse {
                ord.reverse()
            } else {
                ord
            }
        });

        // When unique is enabled, Ok(_) means a truly equal line (same key and
        // same content) already exists — skip insertion.
        let pos = match search_result {
            Ok(_) if self.policy.unique => return false,
            Ok(pos) | Err(pos) => pos,
        };

        let len = self.entries.len();
        let within_top = self.policy.top.is_none_or(|n| len < n || pos < n);
        let within_bottom = self.policy.bottom.is_none_or(|n| len < n || pos > len - n);
        if !within_top || !within_bottom {
            return false;
        }

        self.entries.insert(
            pos,
            Entry {
                key: cached_key.map(String::from),
                line: line.to_string(),
            },
        );
        if let Some(n) = self.policy.top {
            self.entries.truncate(n);
        }
        if let Some(n) = self.policy.bottom
            && self.entries.len() > n
        {
            self.entries.pop_front();
        }
        true
    }

    /// Iterates over the lines in sorted order.
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.line.as_str())
    }

    /// Consumes the buffer, returning the lines in sorted order.
    #[must_use]
    pub fn into_lines(self) -> Vec<String> {
        self.entries.into_iter().map(|e| e.line).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nz(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    #[test]
    fn test_extract_key_field_whitespace_nth_field() {
        assert_eq!(extract_key_field("alpha beta gamma", nz(1), None), "alpha");
        assert_eq!(extract_key_field("alpha beta gamma", nz(2), None), "beta");
        assert_eq!(extract_key_field("alpha beta gamma", nz(3), None), "gamma");
    }

    #[test]
    fn test_extract_key_field_custom_separator_nth_field() {
        assert_eq!(extract_key_field("a:b:c", nz(1), Some(':')), "a");
        assert_eq!(extract_key_field("a:b:c", nz(2), Some(':')), "b");
        assert_eq!(extract_key_field("a:b:c", nz(3), Some(':')), "c");
    }

    #[test]
    fn test_extract_key_field_returns_empty_when_field_missing() {
        assert_eq!(extract_key_field("only one", nz(5), None), "");
        assert_eq!(extract_key_field("a:b", nz(3), Some(':')), "");
    }

    #[test]
    fn test_extract_key_field_multibyte_characters() {
        assert_eq!(
            extract_key_field("日本語 テスト 文字列", nz(2), None),
            "テスト"
        );
        assert_eq!(extract_key_field("α:β:γ", nz(3), Some(':')), "γ");
    }

    fn policy() -> SortPolicy {
        SortPolicy {
            cmp_fn: comparator::compare_normal,
            reverse: false,
            unique: false,
            top: None,
            bottom: None,
            key_field: None,
            field_sep: None,
        }
    }

    fn sort_all(policy: SortPolicy, lines: &[&str]) -> Vec<String> {
        let mut buffer = SortedBuffer::new(policy);
        for line in lines {
            buffer.insert(line);
        }
        buffer.into_lines()
    }

    #[test]
    fn test_sorted_buffer_basic_order() {
        assert_eq!(sort_all(policy(), &["b", "a", "c"]), ["a", "b", "c"]);
    }

    #[test]
    fn test_sorted_buffer_reverse() {
        let p = SortPolicy {
            reverse: true,
            ..policy()
        };
        assert_eq!(sort_all(p, &["b", "a", "c"]), ["c", "b", "a"]);
    }

    #[test]
    fn test_sorted_buffer_unique_skips_duplicates() {
        let p = SortPolicy {
            unique: true,
            ..policy()
        };
        let mut buffer = SortedBuffer::new(p);
        assert!(buffer.insert("a"));
        assert!(!buffer.insert("a"));
        assert!(buffer.insert("b"));
        assert_eq!(buffer.into_lines(), ["a", "b"]);
    }

    #[test]
    fn test_sorted_buffer_top_window() {
        let p = SortPolicy {
            top: Some(2),
            ..policy()
        };
        let mut buffer = SortedBuffer::new(p);
        assert!(buffer.insert("c"));
        assert!(buffer.insert("a"));
        assert!(buffer.insert("b")); // evicts "c"
        assert!(!buffer.insert("z")); // outside the window
        assert_eq!(buffer.into_lines(), ["a", "b"]);
    }

    #[test]
    fn test_sorted_buffer_bottom_window() {
        let p = SortPolicy {
            bottom: Some(2),
            ..policy()
        };
        let mut buffer = SortedBuffer::new(p);
        assert!(buffer.insert("a"));
        assert!(buffer.insert("c"));
        assert!(buffer.insert("b")); // evicts "a"
        assert!(!buffer.insert("0")); // outside the window
        assert_eq!(buffer.into_lines(), ["b", "c"]);
    }

    #[test]
    fn test_sorted_buffer_zero_window_rejects_everything() {
        let p = SortPolicy {
            top: Some(0),
            ..policy()
        };
        let mut buffer = SortedBuffer::new(p);
        assert!(!buffer.insert("a"));
        assert!(buffer.into_lines().is_empty());
    }

    #[test]
    fn test_sorted_buffer_key_field_with_separator() {
        let p = SortPolicy {
            key_field: Some(nz(2)),
            field_sep: Some(':'),
            ..policy()
        };
        assert_eq!(
            sort_all(p, &["banana:2", "apple:3", "cherry:1"]),
            ["cherry:1", "banana:2", "apple:3"]
        );
    }

    #[test]
    fn test_sorted_buffer_unique_with_key_keeps_different_content() {
        // Under unique, lines with an equal key but different content are both kept
        let p = SortPolicy {
            unique: true,
            key_field: Some(nz(1)),
            ..policy()
        };
        assert_eq!(sort_all(p, &["a 1", "a 2", "a 1"]), ["a 1", "a 2"]);
    }

    #[test]
    fn test_sorted_buffer_lines_iterates_in_order() {
        let mut buffer = SortedBuffer::new(policy());
        buffer.insert("b");
        buffer.insert("a");
        assert_eq!(buffer.lines().collect::<Vec<_>>(), ["a", "b"]);
    }
}
