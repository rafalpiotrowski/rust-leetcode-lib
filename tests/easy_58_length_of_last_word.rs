#[cfg(test)]
mod tests {
    use leetcode::easy_58_length_of_last_word::*;

    #[test]
    fn test_1() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4)
    }  
    
    #[test]
    fn test_3() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6)
    }  
    
    #[test]
    fn test_4() {
        assert_eq!(length_of_last_word("Today is a nice day".to_string()), 3)
    }  

    #[test]
    fn test_5() {
        assert_eq!(length_of_last_word("a".to_string()), 1)
    }  

    #[test]
    fn test_6() {
        assert_eq!(length_of_last_word("a ".to_string()), 1)
    } 

    #[test]
    fn test_7() {
        assert_eq!(length_of_last_word(" a".to_string()), 1)
    } 
}