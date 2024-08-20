pub struct Solution{}


impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prev = nums[0];

        for i in 1..nums.len() {
            if nums[i] == prev {
                continue;
            }

            res += 1;
            prev = nums[i];
        }

        if nums[0] == 0 {
            res + 1
        } else {
            res
        }
    }
}