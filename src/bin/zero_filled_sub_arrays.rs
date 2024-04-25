use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut total: i64 = 0;
        let mut counter: u32 = 0;
        let mut memo: HashMap<u32, i64> = HashMap::new();
        
        for i in nums {
            match i {
                0 => {
                    counter += 1;
                },
                _ => {
                    if let Some(&val) = memo.get(&counter) {
                        total += val;
                    } else {
                        let res = Solution::combinations(counter);
                        memo.insert(counter, res);
                        total += res;
                    }
                    counter = 0;
                }
            }
        }

        if let Some(&val) = memo.get(&counter) {
            total += val;
        } else {
            total += Solution::combinations(counter);
        }

        total
    }

    fn combinations(n: u32) -> i64 {
        // match n {
        //     0 => {
        //         return 0;
        //     }
        //     1 => {
        //         return 1;
        //     },
        //     2 => {
        //         return 3;
        //     },
        //     _ => {
        //         let n = n as u64;
        //         (((n+1) * n)/2) as i64
        //     }
        // }
        (((n+1) * n)/2) as i64
    }
}

fn main() {
    let nums = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
    println!("{:?}", nums.len());
    // println!("{:?}", Solution::zero_filled_subarray(nums));
    // println!("{:?}", Solution::combinations(3));
}