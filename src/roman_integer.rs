pub struct Solution;


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn multiplier(to_check: Option<char>, special_1: char, special_2: char) -> i32 {
            match to_check {
                Some(c) if c == special_1 || c == special_2 => -1,
                _ => 1,
            }
        }

        let mut res = 0;
        let s: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            let next_char = s.get(i+1).copied();

            match s[i] {
                'M' => res += 1000,
                'D' => res += 500,
                'C' => res += 100 * multiplier(next_char, 'D', 'M'),
                'L' => res += 50,
                'X' => res += 10 * multiplier(next_char, 'L', 'C'),
                'V' => res += 5,
                'I' => res += 1 * multiplier(next_char, 'V', 'X'),
                _ => ()
            }
        }

        res
    }
}