#[cfg(test)]
mod tests {
    use leetcode::easy_67_add_binary::*;

    #[test]
    fn test_1() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100")
    }
}