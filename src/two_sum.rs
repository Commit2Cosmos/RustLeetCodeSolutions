use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        //*                   number, index
        let mut mapping: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (i, n) in nums.into_iter().enumerate() {
            match mapping.get(&n) {
                Some(&k) => return vec![k as i32, i as i32],
                None => {
                    mapping.insert(target-n, i);
                },
            }
        }
        unreachable!();
    }
}