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
    a.chars()
        .flat_map(char::to_uppercase)
        .cmp(b.chars().flat_map(char::to_uppercase))
        .then_with(|| a.cmp(b))
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
}
