pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 11 {
            return Vec::new();
        }

        let mut set: HashSet<&str> = HashSet::new();
        let mut res: HashSet<String> = HashSet::new();

        let chars: Vec<char> = s.chars().collect();

        for i in 0..=chars.len()-10 {
            let slice = &s[i..i+10];
            if set.contains(slice) {
                res.insert(slice.to_owned());
            }
            set.insert(slice);
        }
        
        res.into_iter().collect()
    }
}

fn main() {
    let input: String = format!("AAAAAAAAAAA");
    println!("{:?}", Solution::find_repeated_dna_sequences(input));
}