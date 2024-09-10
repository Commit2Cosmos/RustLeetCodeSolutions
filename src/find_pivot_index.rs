pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        //* calculate prefix sum */
        let sum: i32 = nums.iter().sum();
        let mut left = 0;

        for i in 0..nums.len() {
            if left == sum-left-nums[i] {
                return i as i32;
            }
            left += nums[i];
        }

        -1
    }
}