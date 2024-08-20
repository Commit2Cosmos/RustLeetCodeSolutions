// use std::collections::BinaryHeap;

use std::collections::HashMap;

pub struct Solution{
}

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        // n/2 - max(freq)
        let l = nums.len();

        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut max_freq = (0, 0);

        for i in 0..l/2 {
            let diff = (nums[i] - nums[l-i-1]).abs();
            let v = *map.entry(diff).and_modify(|e| {*e += 1}).or_insert(1);
            if v > max_freq.0 {
                max_freq = (v, diff);
            }
        }

        for i in 0..l/2 {
            if (nums[i] - nums[l-i-1]).abs() != max_freq.1 {
                if nums[i] < max_freq.1 && nums[l-i-1] < max_freq.1 && nums[i].min(nums[l-i-1]) + max_freq.1 <= k {
                    max_freq.0 -= 1;
                }
            }
        }

        (l/2) as i32 - max_freq.0
    }
}