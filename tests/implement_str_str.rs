
#[cfg(test)]
mod tests {
    use leetcode::implement_str_str::*;

    #[test]
    fn test_1() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(str_str("".to_string(), "ll".to_string()), -1)
    }

    #[test]
    fn test_3() {
        assert_eq!(str_str("hello".to_string(), "".to_string()), 0)
    }

    #[test]
    fn test_4() {
        assert_eq!(str_str("".to_string(), "".to_string()), 0)
    }

    #[test]
    fn test_5() {
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1)
    }
}