pub struct Solution;


impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;

        for idx in 0..nums.len() {
            if nums[idx] != 0 {
                nums.swap(i, idx);
                i += 1;
            }
        }
    }
}