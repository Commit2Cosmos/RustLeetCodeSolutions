pub struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn max_profit_recur(prices: Vec<i32>, fee: i32) -> i32 {

        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; 2]; prices.len()+1];
        
        fn recur(prices: &Vec<i32>, fee: i32, idx: usize, has_sold: bool, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if idx == prices.len() {
                return memo[idx][1].unwrap_or(0);
            }

            if let Some(res) = memo[idx][has_sold as usize] {
                return res;
            }

            if has_sold {
                //* buying or not
                memo[idx][has_sold as usize] = Some((-prices[idx]-fee+recur(prices, fee, idx+1, !has_sold, memo)).max(recur(prices, fee, idx+1, has_sold, memo)));
                return memo[idx][has_sold as usize].unwrap();
            }

            //* selling or not
            memo[idx][has_sold as usize] = Some((prices[idx]+recur(prices, fee, idx+1, !has_sold, memo)).max(recur(prices, fee, idx+1, has_sold, memo)));
            memo[idx][has_sold as usize].unwrap()
        }

        recur(&prices, fee, 0, true, &mut memo)
    }

    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {

        let mut after_buy = -prices[0];
        let mut after_sell = 0;

        for i in 1..prices.len() {
            let temp = after_buy;
            after_buy = after_buy.max(after_sell-prices[i]);
            after_sell = after_sell.max(temp+prices[i]-fee);
        }

        after_buy.max(after_sell)
    }
}