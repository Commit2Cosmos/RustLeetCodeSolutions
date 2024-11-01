use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
        let candidates = candidates as usize;

        let (mut left, mut right) = (candidates, costs.len()-1);

        let mut sum: i64 = 0;

        //* true -> left, false -> right
        let mut heap: BinaryHeap<(Reverse<i32>, bool)> = BinaryHeap::new();

        //* insert first and last k values into heap
        for &x in costs.iter().take(candidates) {
            heap.push((Reverse(x), true));
        }

        for &x in costs.iter().rev().take(candidates) {
            if right < left  {
                break;
            }
            heap.push((Reverse(x), false));
            right -= 1;
        }

        // println!("{:?}", heap);

        
        while k > 0 {
            if let Some((value, l)) = heap.pop() {
                sum += value.0 as i64;
                k -= 1;

                if right >= left {
                    if l {
                        heap.push((Reverse(costs[left]), true));
                        left += 1;
                    } else {
                        heap.push((Reverse(costs[right]), false));
                        right -= 1;
                    }
                }
            }

        }

        sum
    }
}