use std::collections::HashSet;

pub struct Solution;


impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        
        let mut res = 0;

        for x in set.iter() {
            if set.contains(&(x - 1)) {
                continue;
            }

            let mut add = 1;
            while set.contains(&(x + add)) {
                add += 1;
            }
            
            res = res.max(add);
        }

        res
    }
}