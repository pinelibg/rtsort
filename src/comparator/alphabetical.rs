use std::cmp::Ordering;

/// Comparison function for standard alphabetical sort
#[must_use]
pub fn compare_normal(a: &str, b: &str) -> Ordering {
    a.cmp(b)
}

/// Comparison function for case-insensitive sort (folds lower case to upper for comparison).
/// When two lines compare equal under case folding, original byte order is used as tiebreaker.
#[must_use]
pub fn compare_ignore_case(a: &str, b: &str) -> Ordering {
    // ASCII fast path: byte-wise folding gives the same result as Unicode
    // uppercasing for ASCII input, without the per-char uppercase iterators
    let folded = if a.is_ascii() && b.is_ascii() {
        a.bytes()
            .map(|byte| byte.to_ascii_uppercase())
            .cmp(b.bytes().map(|byte| byte.to_ascii_uppercase()))
    } else {
        a.chars()
            .flat_map(char::to_uppercase)
            .cmp(b.chars().flat_map(char::to_uppercase))
    };
    folded.then_with(|| a.cmp(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_ignore_case() {
        assert_eq!(compare_ignore_case("apple", "banana"), Ordering::Less);
        assert_eq!(compare_ignore_case("Apple", "banana"), Ordering::Less);
        assert_eq!(compare_ignore_case("apple", "Apple"), Ordering::Greater); // tiebreak: 'a' > 'A'
        assert_eq!(compare_ignore_case("Apple", "apple"), Ordering::Less);
        // "ß".to_uppercase() == "SS" (0x53 0x53) sorts before "T" (0x54)
        assert_eq!(compare_ignore_case("ß", "t"), Ordering::Less);
    }

    #[test]
    fn test_compare_ignore_case_mixed_ascii_and_unicode() {
        // One ASCII and one non-ASCII operand must take the Unicode path
        // and produce the same ordering as the all-Unicode case
        assert_eq!(compare_ignore_case("straße", "STRASSE"), Ordering::Greater); // tiebreak after equal fold
        assert_eq!(compare_ignore_case("STRASSE", "straße"), Ordering::Less);
        assert_eq!(compare_ignore_case("abc", "äbc"), Ordering::Less);
    }

    #[test]
    fn test_compare_ignore_case_ascii_fast_path_consistency() {
        // The ASCII fast path must order exactly like the Unicode path
        let pairs = [("apple", "APPLE"), ("a1", "A2"), ("Z", "a"), ("", "x")];
        for (a, b) in pairs {
            let unicode_ord = a
                .chars()
                .flat_map(char::to_uppercase)
                .cmp(b.chars().flat_map(char::to_uppercase))
                .then_with(|| a.cmp(b));
            assert_eq!(compare_ignore_case(a, b), unicode_ord, "pair: {a:?}, {b:?}");
        }
    }
}
