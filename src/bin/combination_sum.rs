pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}


fn main() {
    let coins: Vec<i32> = [3,5].to_vec();
    let amount: i32 = 12;
    println!("{:?}", Solution::combination_sum(coins, amount));
}