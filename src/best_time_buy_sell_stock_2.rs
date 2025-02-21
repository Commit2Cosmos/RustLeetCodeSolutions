pub struct Solution;


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;

        for right in 1..prices.len() {
            if prices[right] > prices[right-1] {
                res += prices[right] - prices[right-1];
            }
        }

        res
    }
}