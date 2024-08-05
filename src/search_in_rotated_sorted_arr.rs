pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let len = nums.len();

        let mut mid = len/2;

        let (mut left, mut right) = (0, len-1);

        while left < right {
            //* in left portion       
            if nums[mid] >= nums[left] {
                if target > nums[mid] || target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            //* in right portion
            } else {
                if target <= nums[mid] || target > nums[right] {
                    right = mid
                } else {
                    left = mid + 1;
                }
            }

            mid = (left + right) / 2;

            println!("{left}__{mid}__{right}");
        }

        if nums[mid] as i32 == target { mid as i32 } else { -1 }
    }
}

fn main() {
    let nums: Vec<i32> = [5,1,3].to_vec();
    let target: i32 = 1;
    println!("{:?}", Solution::search(nums, target));
}

// [2,3,4,5,6,7,8,1]

// [8,1,2,3,4,5,6,7]