use std::cmp::Ordering;

pub struct Solution;

#[allow(unused_variables)]
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        
        let mut nums: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        nums.sort_by(|a, b| Solution::compare(b, a));
        if nums[0] == "0" {
            return "0".to_string();
        }
        nums.join("")
    }

    fn compare(a: &str, b: &str) -> Ordering {
        (a.to_owned() + b).cmp(&(b.to_owned() + a))
    }
}

fn main() {
    let input: Vec<i32> = vec![00, 0];
    println!("{}", Solution::largest_number(input));
}


