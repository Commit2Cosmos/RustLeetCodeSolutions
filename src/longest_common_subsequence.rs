pub struct Solution;


impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; text2.len()+1]; text1.len()+1];

        fn recur(t1: &[u8], t2: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {

            if i == 0 || j == 0 {
                return 0;
            }

            if let Some(res) = memo[i][j] {
                return res;
            }

            if t1[i-1] == t2[j-1] {
                memo[i][j] = Some(recur(t1, t2, i-1, j-1, memo));
                return 1 + memo[i][j].unwrap();
            }

            memo[i][j] = Some(recur(t1, t2, i-1, j, memo).max(recur(t1, t2, i, j-1, memo)));
            memo[i][j].unwrap()
        }

        recur(text1.as_bytes(), text2.as_bytes(), text1.len(), text2.len(), &mut memo)
    }
}