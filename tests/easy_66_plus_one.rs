#[cfg(test)]
mod tests {
    use leetcode::easy_66_plus_one::*;

    #[test]
    fn test_1() {
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4])
    }

    #[test]
    fn test_2() {
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2])
    }

    #[test]
    fn test_3() {
        assert_eq!(plus_one(vec![9]), vec![1,0])
    }

    #[test]
    fn test_4() {
        assert_eq!(plus_one(vec![9,9]), vec![1,0,0])
    }

    #[test]
    fn test_5() {
        assert_eq!(plus_one(vec![9,0,9]), vec![9,1,0])
    }

    #[test]
    fn test_6() {
        assert_eq!(plus_one(vec![9,9,9,9]), vec![1,0,0,0,0])
    }
}