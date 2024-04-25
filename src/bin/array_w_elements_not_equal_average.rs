pub struct Solution;

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {

        nums.sort();

        let mut res: Vec<i32> = vec![];

        let (mut l, mut r) = (0, nums.len()-1) ;

        while res.len() < nums.len() {

            res.push(nums[l]);
            l += 1;

            if l <= r {
                res.push(nums[r]);
                r -= 1;
            }

        }

        res
    }
}

fn main() {
    let nums: Vec<i32> = [1,2,3,4,5].to_vec();
    println!("{:?}", Solution::rearrange_array(nums));
}


// 5,2,3
// 4,1