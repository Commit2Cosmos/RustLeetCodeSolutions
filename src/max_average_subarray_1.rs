pub struct Solution;


impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum: i32 = nums[..k as usize].iter().sum();
        let mut res = sum;

        for (i, x) in nums[k as usize..].iter().enumerate() {
            sum += x - nums[i];
            res = res.max(sum);
        }

        res as f64 / k as f64
    }
}