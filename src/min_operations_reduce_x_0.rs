pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {

        let len = nums.len();

        let tot: i32 = nums.iter().sum();
        let mut res: usize = usize::MAX;

        if tot == x { res = len };

        let mut sum: i32 = 0;
        let mut left: usize = 0;

        for right in 0..len {
            sum += nums[right];

            while tot - sum < x && left < right {
                sum -= nums[left];
                left += 1;
            }

            if tot - sum == x {
                res = res.min(len - right + left - 1);
            }
        }

        if res != usize::MAX { res as i32 } else { -1 }
    }
}

fn main() {
    let nums: Vec<i32> = [8828,9581,49,9818,9974,9869,9991,10000,10000,10000,9999,9993,9904,8819,1231,6309].to_vec();
    let x: i32 = 134365;
    println!("{:?}", Solution::min_operations(nums, x));
}