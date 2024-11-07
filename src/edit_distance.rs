pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {

        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; word2.len()+1]; word1.len()+1];
        
        fn recur(w1: &[u8], w2: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if i == w1.len() {
                return (w2.len() - j) as i32;
            }

            if j == w2.len() {
                return (w1.len() - i) as i32;
            }

            if let Some(res) = memo[i][j] {
                return res;
            }

            if w1[i] == w2[j] {
                memo[i][j] = Some(recur(w1, w2, i+1, j+1, memo));
                return memo[i][j].unwrap();
            }
            
            //* remove the letter | change the letter
            memo[i][j] = Some(1 + recur(w1, w2, i+1, j, memo).min(recur(w1, w2, i+1, j+1, memo)).min(recur(w1, w2, i, j+1, memo)));
            memo[i][j].unwrap()
        }

        recur(word1.as_bytes(), word2.as_bytes(), 0, 0, &mut memo)
    }

    #[allow(dead_code)]
    pub fn min_distance_memo(word1: String, word2: String) -> i32 {
        let mut memo: Vec<Vec<i32>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        for i in (0..=word1.len()).rev() {
            for j in (0..=word2.len()).rev() {
                if i == word1.len() || j == word2.len() {
                    memo[i][j] = (word1.len() - i + word2.len() - j) as i32;
                } else if word1[i] == word2[j] {
                    memo[i][j] = memo[i + 1][j + 1];
                } else {
                    memo[i][j] = 1 + (memo[i+1][j+1].min(memo[i+1][j]).min(memo[i][j+1]));
                    memo[i][j] = 1 + (memo[i + 1][j + 1].min(memo[i + 1][j]).min(memo[i][j + 1]));
                }
            }
        }
        memo[0][0]
    }
}