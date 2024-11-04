pub struct Solution;


impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];

        match nums.len() {
            1 => return nums[0],
            2 => return nums[0].max(nums[1]),
            3 => return (nums[0]+nums[2]).max(nums[1]),
            _ => {
                dp[0] = nums[0];
                dp[1] = nums[1];
                dp[2] = nums[2] + dp[0];
        
                for i in 3..dp.len() {
                    dp[i] = nums[i] + dp[i-2].max(dp[i-3])
                }
        
                dp[dp.len()-1].max(dp[dp.len()-2])
            }
        }
    }
}