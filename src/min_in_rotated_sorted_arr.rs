pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {

        let len = nums.len();

        if len == 1 { return nums[0]; }
        
        let mut mid = len/2;

        let (mut left, mut right) = (0, len-1);

        while left < right {
            if nums[mid] > nums[right] {
                left = mid + 1;
                mid = (mid + right) / 2;
            } else {
                right = mid;
                mid = (mid + left) / 2;
            }

            println!("{left}__{mid}__{right}");
        }

        nums[left]
    }
}

fn main() {
    let nums: Vec<i32> = [3,1,2].to_vec();
    println!("{:?}", Solution::find_min(nums));
}

// [2,3,4,5,6,7,8,1]

// [8,1,2,3,4,5,6,7]