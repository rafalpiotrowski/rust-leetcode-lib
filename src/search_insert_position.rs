pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
         return 0;
    }

    let mut low = 0;
    let mut high = (nums.len() as i32) - 1;
    let mut mid = 0;

    while low <= high {
        mid = (low + high) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        if target < nums[mid as usize] {
            if mid == 0 {
                return 0;
            }
            if target > nums[((mid-1) as usize)] {
                return mid;
            }
            high = mid - 1;
        }
        else if target > nums[(mid as usize)] {
            if mid == (nums.len() as i32) -1 {
                return mid+1;
            }
            if target < nums[((mid+1) as usize)] {
                return mid+1;
            }
            low = mid + 1;
        }
    }
    -1
}

pub fn search_insert_brute_force(nums: Vec<i32>, target: i32) -> i32 {
    let mut index = 0;
    for i in nums {
        if i >= target {
            return index;
        }
        index += 1;
    }
    index
}