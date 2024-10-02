pub struct Solution;


impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut l = 0;
        let mut res: i32 = 0;

        for idx in 0..nums.len() {
            
            if nums[idx] == 0 {
                k -= 1;
            }
            
            while k < 0 {
                if nums[l] == 0 {
                    k += 1;
                }
                l += 1;
            }

            let curr_len: i32 = idx as i32 - l as i32 +1;
            if curr_len > res {
                res = curr_len;
            }
        }

        res as i32
    }
}