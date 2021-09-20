use std::convert::TryInto;

/*
Runtime: 3 ms, faster than 61.54% of Rust online submissions for Implement strStr().
Memory Usage: 2.1 MB, less than 97.20% of Rust online submissions for Implement strStr()
 */
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 {
        return 0;
    }   

    if haystack.len() == 0 {
        return -1;
    }

    match haystack.find(needle.as_str()) {
        None => return -1,
        Some(i) => return i.try_into().unwrap()
    }
}