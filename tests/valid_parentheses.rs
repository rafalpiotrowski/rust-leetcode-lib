
#[cfg(test)]
mod tests {
    use leetcode::valid_parentheses::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("()".to_string()), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid("[[[".to_string()), false)
    }
}