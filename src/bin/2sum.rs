use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        //*                   leftover, index
        let mut mapping: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            if let Some(k) = mapping.get(n) {
                return vec![*k as i32, i as i32];
            }
            mapping.insert(target-n, i);
        }

        unreachable!()
    }
}

fn main() {
    let nums = [3,2,4].to_vec();
    let target = 6;
    println!("{:?}", Solution::two_sum(nums, target));
}