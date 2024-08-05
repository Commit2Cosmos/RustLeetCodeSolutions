pub struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        
        nums.sort_unstable();

        let m = 1_000_000_007;


        //* fast exponentiaition
        let fast_pow = |mut n: usize| -> usize {
            let mut ret = 1;
            let mut base = 2;
            while n > 0 {
                if n & 1 == 1 {
                    ret = ret * base % m;
                }
                n >>= 1;
                base = base * base % m;
            }
            ret
        };

        
        let mut res = 0;
        
        for i in 0..nums.len() {
            if 2 * nums[i] > target {
                break;
            }

            let mut l = i;
            let mut r = nums.len();
            let target = target - nums[i];

            while l < r {
                let mid = l + (r - l) / 2;

                if nums[mid] > target {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            
            res = (res + fast_pow(l - i - 1)) % m;
        }

        res as i32
    }
}

fn main() {
    let nums: Vec<i32> = [2,3,3,4,6,7].to_vec();
    let target: i32 = 12;
    println!("{:?}", Solution::num_subseq(nums, target));

}

// 1 -> 1 + 1
// 2 -> 1 + 3
// 3 -> 1 + 6
// 4 -> 1 + 10
// 5 -> 1 + 15