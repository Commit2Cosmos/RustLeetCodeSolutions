pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        let mut right: usize = 1;
        let mut flag = true;

        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                nums[right] = nums[i];
                right += 1;
                flag = true;
            } else if flag {
                nums[right] = nums[i];
                flag = false;
                right += 1;
            }
        }

        right as i32
    }
}