use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut res: HashSet<Vec<i32>> = HashSet::new();
        let right_default = nums.len()-1;

        nums.sort();

        // println!("{:?}", nums);

        for i in 0..nums.len()-2 {
            let ith = nums[i];

            if i > 0 && ith == nums[i - 1] {
                continue;
            }

            let mut left = i+1;
            let mut right = right_default;

            
            while left < right {
                
                let num_l = nums[left];
                let num_r = nums[right];

                match ith + num_l + num_r {
                    d if d < 0 => left += 1,
                    d if d > 0 => right -= 1,
                    0 => {
                        res.insert(vec![ith, num_l, num_r]);

                        while (left < right) && (nums[left] == num_l) {
                            left += 1;
                        }
                        while (left < right) && (nums[right] == num_r) {
                            right -= 1;
                        }

                    },
                    _ => unreachable!()
                }
            }
        }

        res.into_iter().collect()
    }
}

fn main() {
    let nums = [-1,0,1,2,-1,-4].to_vec();
    // let nums = [0,0,0,0,0].to_vec();
    println!("{:?}", Solution::three_sum(nums));
}