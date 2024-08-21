use std::collections::HashMap;


pub struct Solution {}


impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // if nums.len() <= k as usize {
        //     if nums.un
        // }
        
        let k = k as usize;
        
        let mut seen: HashMap<i32, usize> = HashMap::new();

        nums.into_iter().enumerate()
        .any(|(i, x)| {
            if let Some(j) = seen.insert(x, i) {
                i-j <= k
            } else {
                false
            }
        })
    }
}