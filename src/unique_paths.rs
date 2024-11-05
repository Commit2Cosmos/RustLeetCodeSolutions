pub struct Solution;


impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {

        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; n as usize]; m as usize];

        fn recur(m: i32, n: i32, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if m < 0 || n < 0 {
                return 0;
            }
            if m == 0 && n == 0 {
                return 1;
            }
            if let Some(res) = memo[m as usize][n as usize] {
                return res;
            }
            memo[m as usize][n as usize] = Some(recur(m-1, n, memo) + recur(m, n-1, memo));
            memo[m as usize][n as usize].unwrap()
        }

        recur(m-1, n-1, &mut memo)
    }
}