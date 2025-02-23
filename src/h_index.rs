pub struct Solution;


impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let max_value = *citations.iter().max().unwrap() as usize;
        let mut counted_cit: Vec<usize> = vec![0; max_value+1];

        for c in citations {
            counted_cit[c as usize] += 1;
        }

        let mut counter = 0;
        for (i, c) in (counted_cit.into_iter().enumerate()).rev() {
            counter += c;
            if counter >= i {
                return i as i32;
            }
        }

        0
    }
}