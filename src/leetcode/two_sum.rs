

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r = Vec::new();
    let length = nums.len();
    for x in 0..length {
        for y in 0..length {
            if x != y {
                let v1 = nums[x];
                let v2 = nums[y];
                if v1 + v2 == target {
                    r.insert(0, x as i32);
                    r.insert(1, y as i32);
                    return r;
                }
            }
        }
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

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