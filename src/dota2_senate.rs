use std::collections::VecDeque;

pub struct Solution;


impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut counter: i32 = 0;
        let mut senate: VecDeque<_> = senate.chars().collect();

        while counter.abs() != senate.len() as i32 {
            match senate.pop_front().unwrap() {
                x if x == 'R' => {
                    if counter >= 0 {
                        senate.push_back(x);
                    }
                    counter += 1;
                },
                x if x == 'D' => {
                    if counter <= 0 {
                        senate.push_back(x);
                    }
                    counter -= 1;
                },
                _ => unreachable!()
            }
        }

        if counter > 0 { "Radiant".to_string() } else { "Dire".to_string() }
    }
}