
#[cfg(test)]
mod tests {
    use leetcode::palindrome_number::*;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(123), false)
    }

    #[test]
    fn test_4() {
        assert_eq!(is_palindrome(1234567899), false)
    }

    #[test]
    fn test_5() {
        assert_eq!(is_palindrome(1234554321), true)
    }
}