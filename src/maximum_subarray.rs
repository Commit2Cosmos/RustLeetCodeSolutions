
pub struct Solution;


impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        let mut res = nums[0];
        let mut curr_sum = 0;

        for n in nums {

            curr_sum = curr_sum.max(0) + n;

            if curr_sum > res {
                res = curr_sum;
            }
        }

        res
    }
}