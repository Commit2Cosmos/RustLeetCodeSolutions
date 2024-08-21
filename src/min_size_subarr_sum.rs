pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_length: usize = usize::MAX;

        let mut curr_sum = 0;
        let mut left: usize = 0;

        nums.iter().enumerate().for_each(|(i, x)| {
            curr_sum += x;
            while curr_sum >= target {
                min_length = min_length.min(i-left);
                curr_sum -= nums[left];
                left += 1;
            }
        });

        if min_length > nums.len() { 0 } else { min_length as i32 + 1 }
    }
}