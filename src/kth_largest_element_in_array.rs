pub struct Solution;

use::std::collections::BinaryHeap;


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);

        for _ in 0..k-1 {
            heap.pop().unwrap();
        }

        heap.pop().unwrap()
    }

    #[allow(dead_code)]
    pub fn find_kth_largest_quick(nums: Vec<i32>, k: i32) -> i32 {
        fn recur(numbers: &Vec<i32>, k: usize) -> i32 {
            let pivot = numbers[0];
            let (mut less, mut equal, mut greater) = (Vec::new(), Vec::new(), Vec::new());

            for &n in numbers {
                match n.cmp(&pivot) {
                    std::cmp::Ordering::Less => less.push(n),
                    std::cmp::Ordering::Equal => equal.push(n),
                    std::cmp::Ordering::Greater => greater.push(n),
                }
            }

            if greater.len() >= k {
                return recur(&greater, k);
            } else if greater.len() + equal.len() >= k {
                return pivot;
            }

            recur(&less, k - greater.len() - equal.len())
        }

        recur(&nums, k as usize)
    }
}