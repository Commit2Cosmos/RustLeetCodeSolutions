pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;

        let mut max_a: i32 = 0;

        while left < right {
            max_a = std::cmp::max((right - left) as i32 * std::cmp::min(height[left], height[right]), max_a);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_a
    }
}

fn main() {
    let height: Vec<i32> = [1,1].to_vec();
    println!("{:?}", Solution::max_area(height));
}