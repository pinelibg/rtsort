pub mod comparator;

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
}
