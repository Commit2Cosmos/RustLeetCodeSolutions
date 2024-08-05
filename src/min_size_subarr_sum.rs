pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        
        let mut left: usize = 0;

        let mut res = usize::MAX;
        let mut count = 0;
        let mut sum = 0;

        for right in 0..nums.len() {
            sum += nums[right];
            count += 1;

            while sum >= target {
                res = res.min(count);
                sum -= nums[left];
                count -= 1;
                left += 1;
            }
        }

        if res as i32 == -1 { 0 } else { res as i32 }
    }
}

fn main() {
    let target: i32 = 11;
    let nums: Vec<i32> = [1,1,1,1,1,1,1,1].to_vec();
    println!("{:?}", Solution::min_sub_array_len(target, nums));
}