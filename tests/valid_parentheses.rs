
#[cfg(test)]
mod tests {
    use leetcode::valid_parentheses::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("()".to_string()), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid("()[]{}".to_string()), true)
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("(]".to_string()), false)
    }

    #[test]
    fn test_4() {
        assert_eq!(is_valid("([)]".to_string()), false)
    }

    #[test]
    fn test_5() {
        assert_eq!(is_valid("{[]}".to_string()), true)
    }

    #[test]
    fn test_6() {
        assert_eq!(is_valid("(".to_string()), false)
    }

    #[test]
    fn test_7() {
        assert_eq!(is_valid("((".to_string()), false)
    }
}