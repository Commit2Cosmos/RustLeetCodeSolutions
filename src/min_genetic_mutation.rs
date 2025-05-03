use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, mut bank: Vec<String>) -> i32 {
        let dist = |gene1: &String, gene2: &String| gene1.chars().zip(gene2.chars())
            .filter(|(c1, c2)| c1.ne(&c2))
            .count();

        let mut mutations = 0;
        let mut q = VecDeque::from([start_gene]);

        while !q.is_empty() {
            for _ in 0..q.len() {
                let curr = q.pop_front().unwrap();
                if curr == end_gene {
                    return mutations;
                }

                //* find genes in bank 1 diff away
                let mut idx = 0;
                while idx < bank.len() {
                    if dist(&curr, &bank[idx]) == 1 {
                        q.push_back(bank.swap_remove(idx));
                    } else {
                        idx += 1;
                    }
                }
            }
            mutations += 1;
        }

        -1
    }
}