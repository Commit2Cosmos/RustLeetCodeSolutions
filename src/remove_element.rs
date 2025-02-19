pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut point_l = 0;

        for point_r in 0..nums.len() {
            if nums[point_r] != val {
                nums[point_l] = nums[point_r];
                point_l += 1;
            }
        }
        
        point_l as i32
    }
}