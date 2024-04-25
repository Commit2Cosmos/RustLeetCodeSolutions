pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        
        let k: usize = (k as usize) % nums.len();

        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

fn main() {
    let nums = &mut [1,2,3,4,5,6,7].to_vec();
    let k = 3;
    println!("{:?}", Solution::rotate(nums, k));
}