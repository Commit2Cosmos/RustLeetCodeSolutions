pub struct Solution;


impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut curr = 0;
        
        for x in gain {
            curr += x;
            res = res.max(curr);
        }

        res
    }
}