pub struct Solution;

impl Solution {
    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        
        let mut pointer: usize = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                
                nums[pointer] = nums[i];

                pointer += 1;
            }
            println!("{:?}", nums);
        }

        pointer as i32
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        
        let mut pointer: usize = 1;
        let mut double = false;

        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                nums[pointer] = nums[i];
                pointer += 1;
                double = false;
            } else if !double {
                nums[pointer] = nums[i];
                pointer += 1;
                double = true;
            }

            // println!("{}", pointer);
            // println!("{:?}", nums);
        }

        pointer as i32
    }
}

fn main() {
    let mut nums = [0,0,1,1,1,1,2,2,2,3,3,3,3,3].to_vec();
    println!("{:?}", Solution::remove_duplicates_2(&mut nums));
}