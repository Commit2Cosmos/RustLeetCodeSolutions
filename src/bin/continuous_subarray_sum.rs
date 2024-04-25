use std::collections::HashMap;

pub struct Solution;

#[allow(unused_variables)]
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        //*  remainder: index
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut zero_prev: bool = false;
        for (index, &value) in nums.iter().enumerate() {
            if value == 0 {
                if zero_prev { return true; }
                else {
                    zero_prev = true;
                    continue;
                }
            } else { zero_prev = false; }
            sum += value;
            
            let remainder = sum % k;
            if (remainder == 0) & (index != 0) { return true };
            if let Some(i) = map.insert(remainder, index) {
                if index - i > 1 {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let nums: Vec<i32> = vec![23,2,4,6,7];
    let k: i32 = 6;
    println!("{:?}", Solution::check_subarray_sum(nums, k));
}