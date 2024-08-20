
pub struct Solution {}


impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {

        let mut global_max_sum = nums[0];

        let mut tot = 0;
        let mut global_min_sum = nums[0];

        let mut curr_max_sum = 0;
        let mut curr_min_sum = 0;

        for n in nums {

            curr_max_sum = curr_max_sum.max(0) + n;
            curr_min_sum = curr_min_sum.min(0) + n;
            tot += n;

            if curr_max_sum > global_max_sum {
                global_max_sum = curr_max_sum;
            } else if curr_min_sum < global_min_sum {
                global_min_sum = curr_min_sum;
            }
        }

        if global_max_sum < 0 {
            global_max_sum
        } else {
            global_max_sum.max(tot - global_min_sum)
        }

    }
}