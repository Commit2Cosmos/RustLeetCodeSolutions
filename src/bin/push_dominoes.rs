pub struct Solution;

use std::collections::VecDeque;

impl Solution {

    pub fn push_dominoes(dominoes: String) -> String {
        let mut chars: Vec<char> = dominoes.chars().into_iter().collect();

        //* create double ended queue for Rs and Ls
        let mut deque: VecDeque<(usize, char)> = VecDeque::new();

        //* fill with init values
        for (i, &c) in chars.iter().enumerate() {
            if c != '.' { 
                deque.push_back((i, c));
            }
        }

        while !deque.is_empty() {
            let (i, c) = deque.pop_front().unwrap();
            match c {
                'L' => if i != 0 {
                    if let Some(&c) = chars.get(i-1) {
                        if c == '.' {
                            deque.push_back((i-1, 'L'));
                            chars[i-1] = 'L';
                        }
                    }
                },

                'R' => if let Some(&c) = chars.get(i+1) {
                    if c == '.' {
                        if let Some(&cc) = chars.get(i+2) {
                            if cc != 'L' {
                                deque.push_back((i+1, 'R'));
                                chars[i+1] = 'R';
                            } else {
                                deque.pop_front();
                            }
                        } else {
                            deque.push_back((i+1, 'R'));
                            chars[i+1] = 'R';
                        }
                    }
                },

                _ => ()
            }
        }

        chars.into_iter().collect()
    }

}

fn main() {
    let dominoes = String::from(".LR.L");
    println!("{:?}", Solution::push_dominoes(dominoes));
}