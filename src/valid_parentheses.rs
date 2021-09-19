/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Valid Parentheses.
Memory Usage: 1.9 MB, less than 100.00% of Rust online submissions for Valid Parentheses.
*/
pub fn is_valid(s: String) -> bool {

    if s.len() == 0 || (s.len()%2) != 0 {
        return false;
    }

    let mut stack = vec!();
    let chars = s.chars().collect::<Vec<char>>();

    for c in chars.iter() {
        match c {
            '{' | '[' | '(' => {
                //println!("pushing {}", c);
                stack.push(c)
            },
            _ => {
                match stack.pop() {
                    None => return false,
                    Some(x) => {
                        //println!("pop => c = {}, x = {}", c, x);
                        match (x, c) {
                            ('{', '}') => continue,
                            ('[', ']') => continue,
                            ('(', ')') => continue,
                            (_, _) => return false
                        }
                    }
                }
            }
        }
    }
    stack.len() == 0
}