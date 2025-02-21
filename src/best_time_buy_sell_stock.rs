pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // if prices.len() == 1 {
        //     return 0;
        // }
        let mut min = prices[0];
        let mut res = 0;

        for right in 1..prices.len() {
            min = i32::min(min, prices[right]);
            res = i32::max(prices[right] - min, res);
        }

        res
    }
}