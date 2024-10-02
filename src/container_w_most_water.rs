pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len()-1);
        
        let mut res = 0;

        while l < r {
            res = res.max((r-l) as i32 * height[r].min(height[l]));
            
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}