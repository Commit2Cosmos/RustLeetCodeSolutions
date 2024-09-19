pub struct Solution;


impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        
        let b_len = b.len();

        let mut dp = vec![vec![None; b_len]; 4];

        fn recur(a_idx: usize, b_idx: usize, b_len: usize, a: &Vec<i32>, b: &Vec<i32>, dp: &mut Vec<Vec<Option<i64>>>) -> i64 {
            if a_idx == 4 {
                return 0;
            }

            //* if got to the end of b before a -> definitely unwanted result
            if b_idx == b_len {
                return i64::MIN;
            }

            if let Some(c) = dp[a_idx][b_idx] {
                return c;
            }

            //* skip to next number
            let best = recur(a_idx, b_idx+1, b_len, a, b, dp);

            //* take current number
            dp[a_idx][b_idx] = Some(best.max(recur(a_idx+1, b_idx+1, b_len, a, b, dp).saturating_add(a[a_idx] as i64 * b[b_idx] as i64)));
            dp[a_idx][b_idx].unwrap()
        }

        recur(0, 0, b_len, &a, &b, &mut dp)
    }
}