use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring_hashset(s: String) -> i32 {

        if s.len() == 0 { return 0; }

        let mut res: u16 = 1;
        let mut set: HashSet<char> = HashSet::new();

        let mut left: usize = 0;
        let mut right: usize = 1;

        let s: Vec<char> = s.chars().collect();
        set.insert(s[0]);
        
        while right < s.len() {
            if set.insert(s[right]) {
                res = res.max((right-left+1) as u16);
            } else {
                while !set.insert(s[right]) {
                    set.remove(&s[left]);
                    left += 1;
                }
            }

            // println!("{}, {}", left, right);
            
            right += 1;
        }

        res as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut res: u16 = 0;

        let mut latest_char: [usize; 96] = [0; 96];

        let mut left: usize = 0;

        for (right, ch) in s.chars().enumerate() {
            left = left.max(latest_char[ch as usize - 32]);

            res = res.max((right-left+1) as u16);

            latest_char[ch as usize - 32] = right + 1;
            
            println!("{:?}", latest_char
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value != 0)
            .map(|(_, val)| val)
            .collect::<Vec<_>>());
        }

        res as i32
    }
}

fn main() {
    let s: String = "abcabcbb".to_string();
    println!("{:?}", Solution::length_of_longest_substring(s));
}