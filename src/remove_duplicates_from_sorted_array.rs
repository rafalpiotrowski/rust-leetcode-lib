use std::convert::TryInto;

//use std::convert::TryInto;

/*
Runtime: 6 ms, faster than 25.09% of Rust online submissions for Remove Duplicates from Sorted Array.
Memory Usage: 2.4 MB, less than 24.06% of Rust online submissions for Remove Duplicates from Sorted Array.
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let mut k = 0;
    let mut current_vec_index = 0;
    let mut current_min_val = -101;

    if nums.len() == 0 {
        return 0;
    }

    loop {
        let val = nums[current_vec_index];
        if val > current_min_val {
            current_min_val = val;
            nums[k] = val;
            k = k + 1;
        }

        current_vec_index = current_vec_index + 1;
        if current_vec_index == nums.len() {
            return k.try_into().unwrap();
        }
    }
}