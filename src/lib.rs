/// ## Examples
/// ```rust
/// use lib::*;
/// assert!(matches("a", "a b") == true);
/// assert!(matches("c", "a b") == false);
/// ```
pub fn matches(pattern: &str, content: &str) -> bool {
    content.contains(pattern)
}

#[cfg(test)]
mod matches {
    use super::*;

    #[test]
    fn with_matching_content() {
        assert!(matches("lorem", "lorem ipsum"));
    }

    #[test]
    fn with_no_matching_content() {
        assert!(!matches("dolor", "lorem ipsum"));
    }
}
