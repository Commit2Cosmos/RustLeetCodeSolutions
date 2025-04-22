use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        
        for x in strs {
            let mut y = x.clone().into_bytes();
            y.sort();
            anagrams.entry(y).or_default().push(x);
        }

        anagrams.into_values().collect()
    }
}