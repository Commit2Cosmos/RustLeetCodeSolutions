use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;


impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut pairs: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        
        let mut nums1_sum: i64 = 0;
        let mut res: i64 = 0;

        let k = k as usize;

        for (n1, n2) in pairs {
            nums1_sum += n1 as i64;
            heap.push(Reverse(n1));

            if heap.len() > k {
                nums1_sum -= heap.pop().unwrap().0 as i64;
            }

            if heap.len() == k {
                res = res.max(nums1_sum as i64 * n2 as i64);
            }
        }
        res
    }
}