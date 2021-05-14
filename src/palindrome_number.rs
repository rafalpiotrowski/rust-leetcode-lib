
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { 
        return false
    } else if x >= 0 && x <= 9 {
        return true
    }

    let s1 = x.to_string().chars().collect::<Vec<char>>();
    let s2 = x.to_string().chars().rev().collect::<Vec<char>>();
    for x in 0..&s1.len()-1 {
        if s1[x] != s2[x] {
            return false
        } 
    }
    true
}

pub fn is_palindrome2(x: i32) -> bool {
    if x < 0 { 
        return false
    } else if x >= 0 && x <= 9 {
        return true
    }

    let s1 = x.to_string().chars().collect::<Vec<char>>();
    let half_size = s1.len() / 2;
    let length = s1.len() - 1;
    for x in 0..half_size {
        if s1[x] != s1[length - x] {
            return false
        } 
    }
    true
}
