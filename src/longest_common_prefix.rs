use std::cmp;

/*
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".
Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
 

Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.

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