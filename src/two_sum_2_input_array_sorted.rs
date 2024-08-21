pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len()-1;
        
        loop {
            match (numbers[left]+numbers[right]).cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
            }
        }
    }
}