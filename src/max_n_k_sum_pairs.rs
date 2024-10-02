pub struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let (mut l, mut r) = (0, nums.len()-1);

        let mut res = 0;

        while l < r {
            match (nums[r] + nums[l]).cmp(&k) {
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {
                    l += 1;
                    r -= 1;
                    res += 1;
                },
            }
        }

        res
    }
}