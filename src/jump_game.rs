pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut counter = 1;

        for pointer in (0..nums.len()-1).rev() {
            if nums[pointer] >= counter {
                counter = 1;
            } else {
                counter += 1;
            }
        }

        if counter > 1 {
            return false;
        }
        true
    }
}