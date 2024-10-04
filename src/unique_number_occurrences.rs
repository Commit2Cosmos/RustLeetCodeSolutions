use std::collections::{HashMap, HashSet};

pub struct Solution;


impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::with_capacity(arr.len());

        for x in arr {
            *map.entry(x).or_insert(0) += 1;
        }

        let mut set = HashSet::new();
        for x in map.into_values() {
            if !set.insert(x) {
                return false;
            }
        }

        true
    }
}