pub fn prefix(s: &str, max_bytes: usize) -> &str {
    let mut end = max_bytes.min(s.len());
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s.get(..end).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::prefix;

    #[test]
    fn keeps_short_strings_whole() {
        assert_eq!(prefix("abc", 10), "abc");
    }

    #[test]
    fn cuts_at_byte_limit() {
        assert_eq!(prefix("abcdef", 3), "abc");
    }

    #[test]
    fn backs_off_to_char_boundary() {
        assert_eq!(prefix("aé", 2), "a");
    }
}
