pub struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        let mut changed = false;
        
        for i in 0..nums.len()-1 {
            if nums[i] > nums[i+1] {

                if changed {
                    return false;
                }
                changed = true; 
                
                if i != 0 {
                    if let Some(&e1) = nums.get(i+2) {
                        if (nums[i-1] > nums[i+1]) & (nums[i] >= e1) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let input = [1,3,5,2,4].to_vec();
    println!("{:?}", Solution::check_possibility(input));
}