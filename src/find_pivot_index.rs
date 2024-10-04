pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut presum = 0;
        let sum: i32 = nums.iter().sum();

        for (idx, x) in nums.into_iter().enumerate() {
            if sum - x - presum == presum {
                return idx as i32;
            }
            presum += x;
        }

        -1
    }
}