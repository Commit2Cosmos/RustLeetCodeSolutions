pub struct NumArray {
    prefix: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len()];
        prefix[0] = nums[0];

        for (i, &n) in nums.iter().enumerate().skip(1) {
            prefix[i] = n + prefix[i-1];
        }

        Self { prefix }
    }
    
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize] - if left == 0 {0} else {self.prefix[left as usize-1]}
    }

}