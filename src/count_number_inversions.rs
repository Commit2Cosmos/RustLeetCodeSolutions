use std::collections::HashMap;

pub struct Solution{}


impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let mut requirements_map: HashMap<i32, i32> = HashMap::new();
        for r in requirements.into_iter() {
            requirements_map.insert(r[0], r[1]);
        }

        //* if requirement for 0 is nonsensical */
        if let Some(&k) = requirements_map.get(&0) {
            if k > 0 {
                return 0;
            }
        }

        const MOD: i32 = 10_i32.pow(9) + 7;

        fn process_idx_recur(idx: i32, left_inversions: i32, requirements_map: &HashMap<i32, i32>) -> i32 {
            if idx == 0 {
                if left_inversions > 0 {
                    return 0;
                }
                return 1;
            }
            
            if let Some(&r) = requirements_map.get(&idx) {
                if r != left_inversions {
                    return 0;
                }
            }

            let mut tot = 0;
            for i in 0..=idx.min(left_inversions) {
                tot = (tot + process_idx_recur(idx-1, left_inversions-i, requirements_map)) % MOD;
            }
            tot
        }

        process_idx_recur(n-1, requirements_map[&(n-1)], &requirements_map) % MOD
    }
}