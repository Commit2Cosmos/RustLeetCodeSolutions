pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut res: usize = 0;

        let mut left: usize = 0;
        //      fruit type: total of the type inside window
        let mut set = vec![0; fruits.len()];
        let mut distinct = 0;

        for right in 0..fruits.len() {

            if set[fruits[right] as usize] == 0 {
                distinct += 1;
            }

            set[fruits[right] as usize] += 1;
            
            
            while distinct > 2 {
                set[fruits[left] as usize] -= 1;

                if set[fruits[left] as usize] == 0 {
                    distinct -= 1;
                }

                left += 1;
            }
            
            
            res = res.max(right-left+1);
        }
        
        res as i32
    }
}

fn main() {
    let fruits: Vec<i32> = [0,1,2,2].to_vec();
    println!("{:?}", Solution::total_fruit(fruits));
}