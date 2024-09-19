use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_uniq: HashSet<i32> = HashSet::default();

        nums.into_iter().filter_map(|x| {
            if !nums_uniq.insert(x) {
                return Some(x);
            }
            None
        }).collect()
    }
}