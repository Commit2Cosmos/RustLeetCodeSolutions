pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {

        let k = k as usize;

        let mut left: usize = 0;
        let mut letters = vec![0; 26];

        let mut res = 0;

        let mut max_freq = 0;

        let s = s.into_bytes();

        s.iter().enumerate().for_each(|(i, x)| {
            letters[(x-b'A') as usize] += 1;
            max_freq = max_freq.max(letters[(x-b'A') as usize]);
            if i - left < k + max_freq {
                res = i - left;
            } else {
                letters[(s[left]-b'A') as usize] -= 1;
                left += 1;
            }
        });

        res as i32 + 1
    }
}