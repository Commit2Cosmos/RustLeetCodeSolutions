use std::{cmp::min, collections::VecDeque};

pub struct Solution;


impl Solution {
    // pub fn number_of_substrings_slow(s: String) -> i32 {

    //     let mut res = 0;

    //     let s = s.chars().collect::<Vec<char>>();

    //     for i in 0..s.len() {
    //         let max_zeros: usize = ((s.len()-i) as f32).sqrt() as usize;
    //         let mut num_zeros = 0;
    //         let mut zeros_squared: usize = 0;
    //         let mut j = i;

    //         while num_zeros <= max_zeros && j < s.len() {
                
    //             if s[j] == '0' {
    //                 num_zeros += 1;
    //                 zeros_squared = num_zeros*num_zeros;
    //             } 
    //             if zeros_squared <= (j-i+1)-num_zeros {
    //                 res += 1;
    //             }
    //             j += 1;
    //         }
    //     }

    //     res
    // }

    pub fn number_of_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut res = 0;


        //* substrings with zeros present */
        for k in 1..(s.len() as f32).sqrt() as usize + 1 {
            let mut zeros: VecDeque<usize> = VecDeque::new();
            let mut last_zero: i32 = -1;
            let mut num_ones = 0;

            for right in 0..s.len() {
                if s[right] == '0' {
                    zeros.push_back(right);
                    //* k == max number of zeros before it's too many */
                    while zeros.len() > k {
                        //* remove all ones between last and next zero */
                        num_ones -= zeros[0] as i32 - last_zero - 1;
                        last_zero = zeros.pop_front().unwrap() as i32;
                    }
                } else {
                    num_ones += 1;
                }

                if zeros.len() == k && k.pow(2) <= num_ones as usize {
                    res += min(zeros[0] as i32 - last_zero, num_ones - k.pow(2) as i32 + 1);
                }
            }
        }

        //* substrings of pure ones */
        let mut i = 0;
        while i < s.len() {
            if s[i] == '0' {
                i += 1;
                continue;
            }
            
            let mut summ = 0;
            while i < s.len() && s[i] == '1' {
                summ += 1;
                i += 1;
            }
            res += (summ * (summ + 1)) / 2
        }
        
        res
    }
}