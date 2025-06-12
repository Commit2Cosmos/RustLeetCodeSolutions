pub struct Solution;


impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {

        let root = complexity[0];

        if complexity.iter().skip(1).any(|&x| x <= root) {
            return 0;
        }

        const MOD: i64 = 1_000_000_007;

        let mut res = 1i64;
        for i in 1..=complexity.len() as i64 - 1 {
            res = (res * i) % MOD;
        }

        res as i32
    }
}