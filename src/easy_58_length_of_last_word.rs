/*
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal 
substring
 consisting of non-space characters only.

 

Example 1:

Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.
Example 2:

Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.
Example 3:

Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.
 

Constraints:

1 <= s.length <= 104
s consists of only English letters and spaces ' '.
There will be at least one word in s.
*/

pub fn length_of_last_word(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    match s.trim().chars().rev().collect::<String>().split_once(' ').and_then(|(x, _)| Some(x.len() as i32)) {
        Some(xx) => xx,
        None => s.trim().len() as i32,
    }
}


pub fn length_of_last_word_1(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut index = (s.len() as i32) - 1;
    let mut size = 0;
    let str = s.into_bytes();
    let mut is_counting = false;

    while index >= 0 {
        if is_counting {
            if str[index as usize].is_ascii_whitespace() {
                return size;
            }
            size += 1;
        }
        else //not counting
        {
            if !str[index as usize].is_ascii_whitespace() {
                is_counting = true;
                size += 1;
            }
        }
        index -= 1;
    }

    size
}