use std::cmp::Ordering;

const KB: f64 = 1024.0;
const MB: f64 = KB * 1024.0;
const GB: f64 = MB * 1024.0;
const TB: f64 = GB * 1024.0;
const PB: f64 = TB * 1024.0;
const EB: f64 = PB * 1024.0;

/// Parses a string that might contain a human-readable number with an SI suffix
/// into an `f64` for comparison purposes.
/// If it fails to parse (or has no valid suffix/number), returns None.
pub fn parse_human_numeric(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Find the end of the numeric part
    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
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

    let num_part = &s[..num_end];
    let value: f64 = num_part.parse().ok()?;

    let rest = &s[num_end..];
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

/// Comparison function for standard alphabetical sort
#[must_use]
pub fn compare_normal(a: &str, b: &str) -> Ordering {
    a.cmp(b)
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
    fn test_compare_human_numeric() {
        assert_eq!(
            compare_human_numeric(&"2K".to_string(), &"1M".to_string()),
            Ordering::Less
        );
        assert_eq!(
            compare_human_numeric(&"1.5K".to_string(), &"1500".to_string()),
            Ordering::Greater // 1.5 * 1024 = 1536 > 1500
        );
        assert_eq!(
            compare_human_numeric(&"abc".to_string(), &"1K".to_string()),
            Ordering::Less // Non-numbers come before numbers
        );
    }
}
