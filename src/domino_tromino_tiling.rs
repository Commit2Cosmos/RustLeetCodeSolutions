pub struct Solution;


impl Solution {
    //* 1, 2, 5, 11
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; 5]; n as usize+1];
        
        //* +ve -> xtra space top, -ve -> xtra space bot
        fn recur(n: i32, empty: i32, dp: &mut Vec<Vec<Option<i32>>>) -> i64 {
            
            if n < 0 {
                return 0;
            }

            if n == 0 {
                if empty == 0 {
                    return 1;
                }
                return 0;
            }

            if let Some(k) = dp[n as usize][(empty + 2) as usize] {
                return k as i64;
            }

            let res: i64 = match empty {
                0 => {
                    //* top-left
                    recur(n-1, -1, dp) % MOD +
                    //* bot-left
                    recur(n-1, 1, dp) % MOD +
                    //* bot -
                    recur(n, 2, dp) % MOD +
                    //* |
                    recur(n-1, 0, dp) % MOD
                },
                
                1 => {
                    //* top -
                    recur(n-1, -1, dp) % MOD +
                    //* top-right
                    recur(n-2, 0, dp) % MOD
                },
                
                -1 => {
                    //* bot -
                    recur(n-1, 1, dp) % MOD +
                    //* bot-right
                    recur(n-2, 0, dp) % MOD
                },

                2 => {
                    recur(n-2, 0, dp) % MOD
                },

                -2 => {
                    recur(n-2, 0, dp) % MOD
                },

                _ => unreachable!()
            };

            dp[n as usize][(empty + 2) as usize] = Some((res % MOD) as i32);

            res % MOD
        }

        recur(n, 0, &mut dp) as i32
    }
}