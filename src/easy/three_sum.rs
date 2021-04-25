use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut solutions = Vec::<Vec<i32>>::new();
    let length = nums.len();
    if length < 3 {
        return solutions;
    }
    nums.sort_unstable();
    println!("sorting done!");
    let mut v1;
    let mut v2;
    let mut v3;

    let mut triplets = HashSet::new();

    for i in 0..length-2 {
        v1 = nums[i];
    
        let mut j = i + 1;
        let mut k = length - 1;
        
        while j < k 
        {
            v2 = nums[j];
            v3 = nums[k];
                    
            if v1 + v2 + v3 == 0 {
                        //println!("{} + {} + {} == 0", v1, v2, v3);
                        //println!("current solution pool: {:?}", &solutions);

                        //check for duplicate triplets
                        let t = format!("{}{}{}", &v1, &v2, &v3);

                        //if !solutions.iter().any(|s| s[0] == v1 && s[1] == v2 && s[2] == v3) {
                        if !triplets.contains(&t) {
                            //println!("adding: {:?}", vec!(v1, v2, v3));
                            triplets.insert(t);
                            solutions.push(vec!(v1, v2, v3));
                        }
                j += 1;
                k -= 1;
            } else if v1 + v2 + v3 > 0 {
                k -= 1;
            } else { //v1 + v2 + v3 < 0
                j += 1;
            }
        }
    }
    return solutions;
}
