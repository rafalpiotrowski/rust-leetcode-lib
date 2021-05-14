
#[cfg(test)]
mod tests {
    use leetcode::roman_to_int::*;

    #[test]
    fn test_1() {
        assert_eq!(roman_to_int(String::from("III")), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(roman_to_int(String::from("IV")), 4)
    }

    #[test]
    fn test_3() {
        assert_eq!(roman_to_int(String::from("IX")), 9)
    }

    #[test]
    fn test_4() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58)
    }

    #[test]
    fn test_5() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994)
    }

    #[test]
    fn test_6() {
        assert_eq!(roman_to_int(String::from("MMMCC")), 3200)
    }
}