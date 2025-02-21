use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let n = nums.len()/2;
        for e in nums {
            map.entry(e).and_modify(|e| *e += 1).or_insert(1);
            if *map.get(&e).unwrap() > n {
                return e;
            }
        }

        unreachable!();
    }
}