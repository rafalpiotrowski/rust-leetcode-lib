#[cfg(test)]
mod test {
    use leetcode::longest_common_prefix::*;

    #[test]
    fn empty_lcp() {
        assert_eq!(longest_common_prefix(vec![]), "");
    }

    #[test]
    fn single_lcp() {
        assert_eq!(longest_common_prefix(vec!["ab".to_string()]), "ab");
    }

    #[test]
    fn no_lcp() {
        assert_eq!(longest_common_prefix(vec!["a".to_string(), "b".to_string(), "c".to_string()]), "");
    }

    #[test]
    fn valid_lcp() {
        // assert_eq!(longest_common_prefix(vec!["aa", "ab", "ac"]), "a");
        assert_eq!(longest_common_prefix(vec!["aba".to_string(), "abb".to_string(), "abc".to_string()]), "ab");
    }

    #[test]
    fn valid_is_shortest_lcp() {
        assert_eq!(longest_common_prefix(vec!["aba".to_string(), "ab".to_string(), "abc".to_string()]), "ab");
    }

    #[test]
    fn valid_is_shortest_test1() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
    }

    #[test]
    fn valid_is_shortest_test2() {
        assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
    }
}