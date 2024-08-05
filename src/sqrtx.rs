pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        if x < 4 {
            return 1;
        }
        
        let mut lower = 2;
        let mut higher = 46340.min(x >> 1);

        if higher * higher <= x {
            return higher;
        }
        
        while higher - lower > 1 {
            let mid = lower + ((higher - lower) / 2);
            let pow = mid * mid;
            match pow.cmp(&x) {
                Ordering::Less => {
                    lower = mid;
                },
                Ordering::Equal => {
                    return mid;
                },
                Ordering::Greater => {
                    higher = mid;
                }
            }
        }
        lower
    }
}

fn main() {
    let num = 2147395599;
    println!("{}", Solution::my_sqrt(num));
}