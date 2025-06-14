use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {

        //*                   (index, min, max)
        let mut memo: HashMap<(usize, i32, i32), i64> = HashMap::new();

        fn count_recur(nums: &Vec<i32>, curr_min: i32, curr_max: i32, i: usize, k: i32, memo: &mut HashMap<(usize, i32, i32), i64>) -> i64 {
            if i == nums.len() { return 1; }

            const MOD: i64 = 1_000_000_007;

            let new_max = curr_max.max(nums[i]);
            let new_min = curr_min.min(nums[i]);

            if let Some(&res) = memo.get(&(i, new_min, new_max)) {
                return res as i64;
            }

            let res = (count_recur(nums, nums[i], nums[i], i+1, k, memo) + if (new_max - new_min).abs() <= k { count_recur(nums, new_min, new_max, i+1, k, memo) } else { 0 }) % MOD;
            
            memo.insert((i, new_min, new_max), res);

            return res;
        }

        count_recur(&nums, nums[0], nums[0], 1, k, &mut memo) as i32
    }
}