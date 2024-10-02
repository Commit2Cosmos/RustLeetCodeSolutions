pub struct Solution;


impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut s1 = i32::MAX;
        let mut s2 = i32::MAX;

        for x in nums {
            if x > s2 { return true; }
            if x > s1 { s2 = s2.min(x) }
            s1 = s1.min(x);
        }

        return false;
    }
}