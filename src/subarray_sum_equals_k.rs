use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut prefix: HashMap<i32, usize> = HashMap::from([(0, 1)]);
        let mut sum = 0;

        let mut res = 0;

        for i in 0..nums.len() {
            sum += nums[i];

            if let Some(j) = prefix.get(&(sum-k)) {
                res += j;
            }

            prefix.entry(sum).and_modify(|x| *x += 1).or_insert(1);
        }

        res as i32
    }
}