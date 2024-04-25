pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {

        nums.sort_unstable();

        let mut res = 0;
        let mut tot: i64 = 0;
        let mut left: usize = 0;
        
        for right in 0..nums.len() {
            tot += nums[right] as i64;

            while nums[right] as i64 * (right - left + 1) as i64 > tot + k as i64 {
                tot -= nums[left] as i64;
                left += 1;
            }

            res = res.max(right-left+1);
        }
        
        res as i32
    }
}

//* sort -> start from max -> init leftover = k -> while loop to next down & check if freq - down < leftover

fn main() {
    let nums: Vec<i32> = [1,4,8,13].to_vec();
    let k: i32 = 5;
    println!("{:?}", Solution::max_frequency(nums, k));
}