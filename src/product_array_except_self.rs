pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut num_zeros = 0;
        let mut product = 1;
        let mut res = vec![0; nums.len()];

        for &n in nums.iter() {
            if n == 0 {
                num_zeros += 1;
                continue;
            }
            product *= n;
        }

        if num_zeros == 1 {
            for (i, &n) in nums.iter().enumerate() {
                if n == 0 {
                    res[i] = product;
                }
            }
        } else if num_zeros == 0 {
            for (i, &n) in nums.iter().enumerate() {
                res[i] = product/n;
            }
        }
        
        res
    }
}