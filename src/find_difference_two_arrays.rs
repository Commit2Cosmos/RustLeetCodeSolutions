use std::collections::HashSet;

pub struct Solution;


impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: HashSet<i32> = nums1.into_iter().collect();
        let nums2: HashSet<i32> = nums2.into_iter().collect();

        vec![nums1.difference(&nums2).map(|x| *x).collect(), nums2.difference(&nums1).map(|x| *x).collect()]
    }
}