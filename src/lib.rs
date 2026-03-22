use std::cmp::Ordering;

const KB: f64 = 1024.0;
const MB: f64 = KB * 1024.0;
const GB: f64 = MB * 1024.0;
const TB: f64 = GB * 1024.0;
const PB: f64 = TB * 1024.0;
const EB: f64 = PB * 1024.0;

/// Parses the numeric prefix of a trimmed string into an `f64`, returning
/// `(value, rest)` where `rest` is the unparsed remainder. Returns `None`
/// if the string does not begin with a valid number.
fn parse_numeric_prefix(s: &str) -> Option<(f64, &str)> {
    if s.is_empty() {
        return None;
    }

    // Find the end of the numeric part (sign only valid at position 0)
    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || (i == 0 && (c == '-' || c == '+')) {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 {
        return None;
    }

    // Handle corner case where the matched string is just a sign
    if num_end == 1 && (s.starts_with('-') || s.starts_with('+')) {
        return None;
    }

    let value: f64 = s[..num_end].parse().ok()?;
    Some((value, &s[num_end..]))
}

/// Parses a string's numeric prefix into an `f64`, ignoring any trailing suffix.
/// Returns `None` if the string does not begin with a valid number.
pub fn parse_numeric(s: &str) -> Option<f64> {
    let s = s.trim();
    parse_numeric_prefix(s).map(|(value, _)| value)
}

/// Parses a string that might contain a human-readable number with an SI suffix
/// into an `f64` for comparison purposes.
/// If it fails to parse (or has no valid suffix/number), returns None.
pub fn parse_human_numeric(s: &str) -> Option<f64> {
    let s = s.trim();
    let (value, rest) = parse_numeric_prefix(s)?;
    let rest_trimmed = rest.trim_start();

    // Suffix typically consists of alphabetic characters (like K, M, G, KiB, etc)
    let mut suffix_end = 0;
    for (i, c) in rest_trimmed.char_indices() {
        if c.is_alphabetic() {
            suffix_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    let suffix_part = &rest_trimmed[..suffix_end];

    let multiplier: f64 = match suffix_part.to_uppercase().as_str() {
        "" | "B" => 1.0,
        "K" | "KI" | "KB" | "KIB" => KB,
        "M" | "MI" | "MB" | "MIB" => MB,
        "G" | "GI" | "GB" | "GIB" => GB,
        "T" | "TI" | "TB" | "TIB" => TB,
        "P" | "PI" | "PB" | "PIB" => PB,
        "E" | "EI" | "EB" | "EIB" => EB,
        _ => {
            // Unrecognized suffix. If it's directly attached without spaces
            // (e.g., "123XYZ"), treat it as not a human number. If there's
            // whitespace after the number (e.g., "123 foo"), the number
            // is "123" and the "foo" is trailing text.
            if rest.starts_with(char::is_whitespace) {
                1.0
            } else {
                return None;
            }
        }
    };

    Some(value * multiplier)
}

/// Comparison function for numeric sort (leading number only, suffix ignored)
#[must_use]
pub fn compare_numeric(a: &str, b: &str) -> Ordering {
    let num_a = parse_numeric(a);
    let num_b = parse_numeric(b);

    match (num_a, num_b) {
        (Some(va), Some(vb)) => va.partial_cmp(&vb).unwrap_or_else(|| a.cmp(b)),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => a.cmp(b),
    }
}

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

/// Comparison function for human-numeric sort
#[must_use]
pub fn compare_human_numeric(a: &str, b: &str) -> Ordering {
    let num_a = parse_human_numeric(a);
    let num_b = parse_human_numeric(b);

    match (num_a, num_b) {
        (Some(va), Some(vb)) => {
            // Compare as f64. If they are exactly equal (or NaN), fallback to string comparison
            va.partial_cmp(&vb).unwrap_or_else(|| a.cmp(b))
        }
        (Some(_), None) => Ordering::Greater, // Non-numbers sort before numbers (matching `sort -h` behavior)
        (None, Some(_)) => Ordering::Less,
        (None, None) => a.cmp(b), // Both are not numbers, fallback to string sort
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric() {
        assert_eq!(parse_numeric("42"), Some(42.0));
        assert_eq!(parse_numeric("10K"), Some(10.0));
        assert_eq!(parse_numeric("2M"), Some(2.0));
        assert_eq!(parse_numeric("-3"), Some(-3.0));
        assert_eq!(parse_numeric("abc"), None);
    }

    #[test]
    fn test_compare_numeric() {
        assert_eq!(compare_numeric("10", "2"), Ordering::Greater);
        assert_eq!(compare_numeric("10K", "2M"), Ordering::Greater); // 10 > 2, suffix ignored
        assert_eq!(compare_numeric("abc", "1"), Ordering::Less);
    }

    #[test]
    fn test_parse_human_numeric() {
        assert_eq!(parse_human_numeric("123"), Some(123.0));
        assert_eq!(parse_human_numeric("1.5"), Some(1.5));
        assert_eq!(parse_human_numeric("2K"), Some(2048.0));
        assert_eq!(parse_human_numeric("1.5M"), Some(1.5 * MB));
        assert_eq!(parse_human_numeric("-5G"), Some(-5.0 * GB));
        assert_eq!(parse_human_numeric("abc"), None);
        assert_eq!(parse_human_numeric("123XYZ"), None);
        assert_eq!(parse_human_numeric("4.0K   ./foo"), Some(4096.0));
        assert_eq!(parse_human_numeric("123 ./foo"), Some(123.0));
        assert_eq!(parse_human_numeric("123 foo"), Some(123.0));
    }

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
    fn test_compare_human_numeric() {
        assert_eq!(compare_human_numeric("2K", "1M"), Ordering::Less);
        assert_eq!(
            compare_human_numeric("1.5K", "1500"),
            Ordering::Greater // 1.5 * 1024 = 1536 > 1500
        );
        assert_eq!(
            compare_human_numeric("abc", "1K"),
            Ordering::Less // Non-numbers come before numbers
        );
    }
}
