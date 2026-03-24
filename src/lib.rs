pub mod comparator;

/// Extracts the Nth (1-indexed) field from a line.
/// When `sep` is `None`, splits on whitespace. When `sep` is `Some(c)`, splits on that character.
/// Returns an empty string if the line has fewer than `n` fields.
pub fn extract_key_field(line: &str, n: usize, sep: Option<char>) -> &str {
    if n == 0 {
        return "";
    }
    match sep {
        None => line.split_whitespace().nth(n - 1).unwrap_or(""),
        Some(c) => line.split(c).nth(n - 1).unwrap_or(""),
    }
}
