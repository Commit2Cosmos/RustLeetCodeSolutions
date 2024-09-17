pub struct NumArray {
    tree: Vec<i32>,
    size: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        let size = nums.len();
        let mut tree = Self { 
            tree: vec![0; size+1],
            size,
        };

        nums.into_iter().enumerate().for_each(|(i, x)| {
            tree.update(i as i32, x)
        });

        tree
    }
    
    pub fn update(&mut self, mut index: i32, val: i32) {
        let diff = Self::sum_range(&self, index, index);
        index += 1;
        while index as usize <= self.size {
            self.tree[index as usize] += val-diff;
            index += index & (-index);
        }
    }

    fn calculate_sum(&self, mut idx: i32) -> i32 {
        let mut sum = 0;

        while idx > 0 {
            sum += self.tree[idx as usize];
            idx -= idx & (-idx);
        }

        sum
    }
    
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.calculate_sum(right+1) - self.calculate_sum(left)
    }
}


// Your NumArray object will be instantiated and called as such:
// let obj = NumArray::new(nums);
// obj.update(index, val);
// let ret_2: i32 = obj.sum_range(left, right);