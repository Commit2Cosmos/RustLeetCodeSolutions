use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {

        let s = s.as_bytes();
        let k = k as usize;

        let mut res = 0;
        let mut counter = 0;
        let mut left = 0;
        let arr = HashSet::from([b'a', b'e', b'i', b'o', b'u']);

        for right in 0..s.len() {

            if arr.contains(&s[right]) {
                counter += 1;
            } 

            if right - left == k {
                if arr.contains(&s[left]) {
                    counter -= 1;
                }
                left += 1;
            }

            res = res.max(counter);
        }

        res
    }


    fn is_vowel(x: u8) -> bool {
        match x {
            b'a' | b'e'| b'i'| b'o'| b'u' => {
                true
            },
            _ => false
        }
    }

    pub fn max_vowels_new(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;

        let count = s.iter().take(k).filter(|&&x| Self::is_vowel(x)).count();       

        s.iter()
            .skip(k)
            .zip(s.iter())
            .fold(
                (count, count),
                // (counter_ahead, counter_behind), (result_ahead, result_behind)
                |acc, x| {
                    let (counter, res) = acc;
                    let (right, left) = x;

                    match (Self::is_vowel(*left), Self::is_vowel(*right)) {
                        (true, false) => (counter - 1, res),
                        (false, true) => (counter + 1, res.max(counter)),
                        _ => (counter, res)
                    }
                }).1 as i32
    }
}

fn main() {
    let s: String = "aeiou".to_string();
    let k: i32 = 2;
    println!("{:?}", Solution::max_vowels_new(s, k));
}