use std::cmp;

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Longest Common Prefix.
Memory Usage: 2.2 MB, less than 35.29% of Rust online submissions for Longest Common Prefix.
 */
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let str0 = &strs[0];
    let str0bytes = str0.as_bytes();
    let mut len = str0.len();
    for str in &strs[1..] {
        len = cmp::min(len,
                       str.as_bytes()
                           .iter()
                           .zip(str0bytes)
                           .take_while(|&(a, b)| a == b)
                           .count());
    }
    (&strs[0][..len]).to_string()
}