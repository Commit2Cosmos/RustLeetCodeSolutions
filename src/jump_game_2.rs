pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut right, mut left) = (0, 0);
        let mut res = 0;

        while right < nums.len()-1 {
            let mut temp = right;
            for i in left..=right {
                temp = usize::max(temp, i + nums[i] as usize);
            }
            left = right + 1;
            right = temp;
            res += 1;
        }


        res
    }
}