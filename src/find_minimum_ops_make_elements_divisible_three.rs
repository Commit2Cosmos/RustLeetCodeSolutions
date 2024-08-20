pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| {
            if x % 3!=0 {
                return acc + 1;
            }
            acc
        })
    }
}