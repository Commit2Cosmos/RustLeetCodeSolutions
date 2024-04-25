pub struct Solution;

impl Solution {
    fn rob_heper(nums: &[i32]) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = 0;

        for n in nums {
            let sum = right.max(left + n);
            left = right;
            right = sum;
        }

        right
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {return nums[0];}
        Self::rob_heper(&nums[1..]).max(Self::rob_heper(&nums[..&nums.len() - 1]))
    }
}

fn main() {
    let nums: Vec<i32> = [1].to_vec();
    println!("{:?}", Solution::rob(nums));
}



// let [left, right] = [0, 0];

//     for (let n of nums) {
//         let sum = Math.max(left + n, right);
//         console.log(sum)
//         left = right;
//         right = sum;
//     }

//     return right