pub struct Solution;

use std::collections::HashSet;


#[allow(unused_variables, unused_mut)]
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set: HashSet<&[u8]> = HashSet::new();
        let expected_len = 2_usize.pow(k as u32);
        
        for window in s.as_bytes().windows(k as usize) {
            // println!("{:?}", window);
            set.insert(window);
            if set.len() == expected_len {
                return true;
            }
        }
        false
    }
    
    pub fn has_all_codes_bytes(s: String, k: i32) -> bool {
        let k: usize = k as usize;
                               //* 2^k
        let expected_len: usize = 1 << k;

        let mut all_codes: Vec<bool> = vec![false; expected_len];
        //* if count == 0 -> return true
        let (mut n, mut count) = (0, expected_len);
                        //* 2^k - 1
        let mask: usize = expected_len - 1;
        // let mut m = 0;

        // println!("init count: {}", count);

        for (i, &byte) in s.as_bytes().iter().enumerate() {
              //* same expressions
            n = ((n*2) % expected_len) + if byte == b'1' { 1 } else { 0 };
            // n = ((n << 1) & mask) + if byte == b'1' { 1 } else { 0 };

            println!("iter: {} ---------------------", i);
            println!("n: {}", n);

            if i + 1 < k || all_codes[n] {
                // println!("skipped");
                continue;
            }

            all_codes[n] = true;
            count -= 1;

            // println!("all_codes: {:?}", all_codes);
            // println!("count: {}", count);

            if count == 0 {
                return true;
            }

        }

        false
    }
}

fn main() {
    let input: String = String::from("00111101");
    let k: i32 = 2;
    println!("{:?}", Solution::has_all_codes_bytes(input, k));
}