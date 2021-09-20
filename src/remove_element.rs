use std::convert::TryInto;

/*
Runtime: 3 ms, faster than 14.50% of Rust online submissions for Remove Element.
Memory Usage: 2.2 MB, less than 21.00% of Rust online submissions for Remove Element.
 */
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut i: usize = 0; //index in vector
    let mut k: usize = 0; // length of the new array
    loop {
        if nums[i] != val { 
            nums[k] = nums[i];
            k = k + 1;
        } 
        i = i + 1;
        if i > nums.len() {
            break;
        }
    }
    k.try_into().unwrap()
}