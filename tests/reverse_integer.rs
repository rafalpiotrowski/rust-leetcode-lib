
#[cfg(test)]
mod tests {
    use leetcode::easy::reverse_integer::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321)
    }
    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321)
    }
    #[test]
    fn test_3() {
        assert_eq!(reverse(-4), -4)
    }
    #[test]
    fn test_4() {
        assert_eq!(reverse(-2147483412), -2143847412)
    }
} // end mod tests
