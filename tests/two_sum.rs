#[cfg(test)]
mod tests {
    use leetcode::easy::two_sum::*;

    #[test]
    fn test1() {
        let nums = vec!(2,7,11,15);
        let target = 9;
        let r = two_sum(nums, target);
        println!("Resutl {:?}", r);
    }

    #[test]
    fn test2() {
        let nums = vec!(3,2,4);
        let target = 6;
        let r = two_sum(nums, target);
        println!("Resutl {:?}", r);
    }

    #[test]
    fn test3() {
        let nums = vec!(3,3);
        let target = 6;
        let r = two_sum(nums, target);
        println!("Resutl {:?}", r);
    }
}