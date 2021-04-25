
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(123), false)
    }

    #[test]
    fn test_4() {
        assert_eq!(is_palindrome(1234567899), false)
    }

    #[test]
    fn test_5() {
        assert_eq!(is_palindrome(1234554321), true)
    }
}