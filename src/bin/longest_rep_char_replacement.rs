pub struct Solution;

// usize - 65

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {

        let s = s.as_bytes();
        let mut num_letters: [usize; 26] = [0; 26];

        let mut res = 0;
        let mut most_common_count = 0;

        let mut left: usize = 0;

        for right in 0..s.len() {
            let last = (s[right] - b'A') as usize;
            num_letters[last] += 1;

            //* compare with last updated one
            most_common_count = most_common_count.max(num_letters[last]);

            if right - left + 1 - most_common_count > k as usize {
                num_letters[(s[left] - b'A') as usize] -= 1;
                left += 1;
            } else {
                res = res.max(right - left + 1);
            }
        }
        res as i32
    }
}

fn main() {
    let s: String = "AABABBA".to_string();
    let k: i32 = 1;
    println!("{:?}", Solution::character_replacement(s, k));
}