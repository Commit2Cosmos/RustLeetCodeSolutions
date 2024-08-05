pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        

        let ans = Self::coin_change_tabulation(&coins, amount);
        
        //*         for each coin, for each amount
        // let mut memo: Vec<Vec<Option<i32>>> = vec![vec![Option::None; amount as usize+1]; coins.len() + 1];
        // let ans = Self::coin_change_recur(&coins, amount, coins.len(), &mut memo)
        if ans >= 10_000_000 { -1 } else { ans }
    }


    fn coin_change_tabulation(coins: &Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![10_000_000; amount as usize + 1];

        dp[0] = 0;
        
        for leftover in 1..amount as usize + 1 {
            for &c in coins {
                if leftover as i32 - c >= 0 {
                    //* because only going up, the dp[leftover-c] is either inf or some value calculated before
                    dp[leftover] = dp[leftover].min(1+dp[leftover-c as usize]);
                }
            }
        }

        dp[amount as usize]
    }


    fn coin_change_recur(coins: &Vec<i32>, amount: i32, coin_index: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {

        if amount == 0 { return 0; }
        
        if coin_index == 0 { return 10_000_000; }

        if let Some(val) = memo[coin_index][amount as usize] {
            return val;
        }

        if coins[coin_index-1] > amount {
            memo[coin_index][amount as usize] = Option::from(Self::coin_change_recur(coins, amount, coin_index-1, memo));
            return memo[coin_index][amount as usize].unwrap();
        }

        memo[coin_index][amount as usize] = Option::from(Self::coin_change_recur(coins, amount, coin_index-1, memo).min(1 + Self::coin_change_recur(coins, amount - coins[coin_index-1], coin_index, memo)));
        memo[coin_index][amount as usize].unwrap()   
    }
}

fn main() {
    let coins: Vec<i32> = [3,5].to_vec();
    let amount: i32 = 12;
    println!("{:?}", Solution::coin_change(coins, amount));
}