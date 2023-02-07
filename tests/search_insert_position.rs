#[cfg(test)]
mod tests {
    use leetcode::search_insert_position::*;

    #[test]
    fn test_1() {
        assert_eq!(search_insert(vec![1,3,5,6], 5), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(search_insert(vec![1,3,5,6], 2), 1)
    }  
    
    #[test]
    fn test_3() {
        assert_eq!(search_insert(vec![1,3,5,6], 7), 4)
    }   

    #[test]
    fn test_4() {
        assert_eq!(search_insert(vec![-1,1,3,5,6,7], 7), 5)
    }  
}