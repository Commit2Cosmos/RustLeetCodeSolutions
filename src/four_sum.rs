use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();
        Solution::k_sum_recur(&nums, target as i64, 4, &mut res, 0, &mut Vec::new());

        res
    }


    fn k_sum_recur(nums: &Vec<i32>, target: i64, sum_num: usize, res: &mut Vec<Vec<i32>>, i: usize, arr: &mut Vec<i32>) {

        if sum_num != 2 {
            //* for loop calling the recur function
            for j in i..nums.len().saturating_sub(sum_num-1) {

                //* eliminate duplicates
                let jth = nums[j];
                if j > i && jth == nums[j - 1] {
                    continue;
                }

                arr.push(jth);
                Solution::k_sum_recur(nums, target-jth as i64, sum_num-1, res, j+1, arr);
                arr.pop();
            }

        } else {
            let mut left = i;
            let mut right = nums.len()-1;

            
            while left < right {
                
                let num_l = nums[left];
                let num_r = nums[right];

                match ((num_l + num_r) as i64).cmp(&target) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        res.push(arr.iter().cloned().chain([num_l, num_r]).collect());

                        while (left < right) && (nums[left] == num_l) {
                            left += 1;
                        }
                    }
                }
            }
        }
    }
}
