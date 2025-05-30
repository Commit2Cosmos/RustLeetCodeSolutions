pub struct Solution;

use std::i32::{MAX, MIN};

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let mut mult = 1;

        let mut chars = s.chars().peekable();

        if let Some(&c) = chars.peek() {
            if c == '+' {
                chars.next();
            } else if c == '-' {
                mult = -1;
                chars.next();
            }
        }

        let s: String = chars.take_while(|x| x.is_ascii_digit()).collect();

        if s.is_empty() {
            return 0;
        }

        match s.parse::<i64>() {
            Ok(num) => {
                let n = num*mult;
                if n > MAX as i64 {
                    MAX
                } else if n < MIN as i64 {
                    MIN
                } else {
                    n as i32
                }
            },
            Err(_) => {
                if mult == 1 {
                    MAX
                } else {
                    MIN
                }
            }
        }
    }
}