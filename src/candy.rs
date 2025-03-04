pub struct Solution;


impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut res: Vec<i32> = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i-1] {
                res[i] = res[i-1] + 1;
            }
        }

        println!("{:?}", res);
        
        for i in (0..ratings.len()-1).rev() {
            if ratings[i] > ratings[i+1] && res[i] <= res[i+1] {
                res[i] = res[i+1] + 1;
            }
        }
        
        println!("{:?}", res);

        res.iter().sum()
    }
}