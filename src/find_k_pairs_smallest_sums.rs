use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {

        let mut res = Vec::new();

        if nums1.is_empty() || nums2.is_empty() || k == 0 {
            return res;
        }

        let mut heap = BinaryHeap::from([(Reverse(1), 0, 0)]);
        let mut set = HashSet::new();
        set.insert((0, 0));

        while k > 0 {
            if let Some((_, i, j)) = heap.pop() {
                res.push(Vec::from([nums1[i], nums2[j]]));

                if i + 1 < nums1.len() && set.insert((i + 1, j)) {
                    heap.push((Reverse(nums1[i+1]+nums2[j]), i+1, j));
                }

                if j + 1 < nums2.len() && set.insert((i, j + 1)) {
                    heap.push((Reverse(nums1[i]+nums2[j+1]), i, j+1));
                }
            }

            k -= 1;
        }

        res
    }
}