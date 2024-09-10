pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut presum = 1;

        let mut res = vec![0; nums.len()];

        for i in 0..nums.len() {
            res[i] = presum;
            presum *= nums[i];
        }

        let mut presum = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= presum;
            presum *= nums[i];
        }

        res
    }
}