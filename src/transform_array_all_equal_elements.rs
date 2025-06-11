pub struct Solution;

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        fn check(nums: &Vec<i32>, k: i32, target: i32) -> bool {
            let mut to_one_count = 0;
            let mut prev_n_one: Option<usize> = None;

            for i in 0..nums.len() {
                if nums[i] == target {
                    if let Some(prev) = prev_n_one {
                        to_one_count += i - prev;
                        prev_n_one = None;
                    } else {
                        prev_n_one = Some(i);
                    }
                }
            }

            k >= to_one_count as i32 && prev_n_one.is_none()
        }

        check(&nums, k, -1) || check(&nums, k, 1)
    }
}