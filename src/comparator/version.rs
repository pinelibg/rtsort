use std::cmp::Ordering;

struct VersionSegments<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for VersionSegments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.remaining.is_empty() {
            return None;
        }

        let in_digits = self.remaining.as_bytes()[0].is_ascii_digit();
        let end = self
            .remaining
            .char_indices()
            .find(|(_, c)| c.is_ascii_digit() != in_digits)
            .map_or(self.remaining.len(), |(i, _)| i);

        let seg = &self.remaining[..end];
        self.remaining = &self.remaining[end..];
        Some(seg)
    }
}

/// Returns an iterator that lazily yields alternating digit/non-digit segments of a
/// version string, stripping a leading `v` or `V` prefix first.
fn version_segments(s: &str) -> impl Iterator<Item = &str> {
    let stripped = s.strip_prefix(['v', 'V']).unwrap_or(s);
    VersionSegments {
        remaining: stripped,
    }
}

/// Compares two ASCII digit segments numerically without parsing them,
/// so arbitrarily long digit runs (beyond `u64` range) are handled correctly.
/// After stripping leading zeros, a longer run is a larger number; equal-length
/// runs compare lexicographically (which matches numeric order for digits).
fn compare_digit_segments(a: &str, b: &str) -> Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

/// Comparison function for version sort (GNU `sort -V` compatible).
/// Numeric segments are compared as integers; non-numeric segments lexicographically.
/// A leading `v`/`V` prefix is stripped before comparison.
#[must_use]
pub fn compare_version(a: &str, b: &str) -> Ordering {
    let mut iter_a = version_segments(a);
    let mut iter_b = version_segments(b);

    loop {
        match (iter_a.next(), iter_b.next()) {
            (None, None) => break,
            (Some(seg_left), Some(seg_right)) => {
                if seg_left == seg_right {
                    continue;
                }

                // Both digit segments: compare numerically
                let left_digits = seg_left.as_bytes().first().is_some_and(u8::is_ascii_digit);
                let right_digits = seg_right.as_bytes().first().is_some_and(u8::is_ascii_digit);

                let ord = if left_digits && right_digits {
                    compare_digit_segments(seg_left, seg_right)
                } else {
                    seg_left.cmp(seg_right)
                };

                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
        }
    }

    // Fall back to lexicographic comparison of the original strings as a tiebreaker
    // to ensure deterministic ordering when version segments are all equal
    // (e.g., "v1.0" vs "1.0" after prefix stripping).
    a.cmp(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_version() {
        // 1.9 < 1.10 (numeric segment comparison, not lexicographic)
        assert_eq!(compare_version("1.9", "1.10"), Ordering::Less);

        // Prefix stripping: "v1.0" vs "1.0" — after stripping both resolve to "1.0",
        // but the original strings differ, so tiebreaker a.cmp(b) applies.
        // "1.0" < "v1.0" lexicographically (b'1' < b'v')
        assert_eq!(compare_version("v1.0", "1.0"), Ordering::Greater);
        assert_eq!(compare_version("1.0", "v1.0"), Ordering::Less);

        // Leading zeros: "1.02" == "1.2" numerically (02 parsed as 2), tiebreaker applies.
        // "1.02" < "1.2" lexicographically
        assert_eq!(compare_version("1.02", "1.2"), Ordering::Less);
        assert_eq!(compare_version("1.2", "1.02"), Ordering::Greater);

        // Pre-release: "1.0-rc1" vs "1.0" — "1.0-rc1" has more segments, so it sorts after "1.0"
        assert_eq!(compare_version("1.0-rc1", "1.0"), Ordering::Greater);
        assert_eq!(compare_version("1.0", "1.0-rc1"), Ordering::Less);

        // Basic ordering chain
        assert_eq!(compare_version("v1.0", "v1.9"), Ordering::Less);
        assert_eq!(compare_version("v1.9", "v1.10"), Ordering::Less);
        assert_eq!(compare_version("v1.10", "v2.0"), Ordering::Less);
    }

    #[test]
    fn test_compare_version_beyond_u64() {
        // Digit runs longer than u64::MAX (~1.8e19) must still compare numerically
        let big = "1.99999999999999999999999";
        let bigger = "1.100000000000000000000000";
        assert_eq!(compare_version(big, bigger), Ordering::Less);
        assert_eq!(compare_version(bigger, big), Ordering::Greater);
        assert_eq!(
            compare_version("1.99999999999999999999999", "1.2"),
            Ordering::Greater
        );
    }
}
