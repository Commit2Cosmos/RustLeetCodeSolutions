pub struct Solution;


impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut res: usize = 0;
        let mut k = 0;

        for (idx, &x) in nums.iter().enumerate() {
            
            if x == 0 {
                k += 1;
            }
            
            while k > 1 {
                if nums[l] == 0 {
                    k -= 1;
                }
                l += 1;
            }

            res = res.max(idx - l);
        }

        res as i32
    }
}