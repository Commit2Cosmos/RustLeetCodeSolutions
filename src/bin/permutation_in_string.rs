pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {

        if s2.len() < s1.len() { return false }

        let mut mapping: [i16; 26] = [0; 26];

        s1.as_bytes().iter().for_each(|&x| mapping[(x - b'a') as usize] -= 1);
        s2[0..s1.len()].as_bytes().iter().for_each(|&x| mapping[(x - b'a') as usize] += 1);

        if mapping.iter().all(|&num| num == 0) { return true; }

        // println!("{:?}", mapping);

        let s2 = s2.as_bytes();

        for right in s1.len()..s2.len() {

            mapping[(s2[right] - b'a') as usize] += 1;
            mapping[(s2[right - s1.len()] - b'a') as usize] -= 1;

            if mapping.iter().all(|&num| num == 0) { return true; }
        }

        false
    }
}

fn main() {
    let s1: String = "ab".to_string();
    let s2: String = "eidboaoo".to_string();

    println!("{:?}", Solution::check_inclusion(s1, s2));
}