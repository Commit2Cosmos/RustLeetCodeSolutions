use std::vec;

pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let len = customers.len();
        // let mut prefix = vec![0; len+1];
        let mut postfix = vec![0; len+1];

        // for (i, c) in customers.chars().enumerate() {
        //     prefix[i+1] = prefix[i] + if c == 'N' {1} else {0};
        // }

        for (i, c) in customers.chars().rev().enumerate() {
            postfix[i+1] = postfix[i] + if c == 'Y' {1} else {0};
        }
        postfix.reverse();

        // println!("{:?}", postfix);



        let mut res = i32::MAX;
        let mut idx: usize = 0;
        let mut pre_val = 0;
        for i in 0..=len {
            pre_val += if i == 0 {0} else if customers.chars().nth(i-1).unwrap() == 'N' {1} else {0};
            let sum = postfix[i] + pre_val;
            println!("{}", sum);
            if res > sum {
                res = sum;
                idx = i;
            }
        }

        idx as i32
    }
}

fn main() {
    let input: String = "YYYY".to_string();
    println!("{:?}", Solution::best_closing_time(input));
}