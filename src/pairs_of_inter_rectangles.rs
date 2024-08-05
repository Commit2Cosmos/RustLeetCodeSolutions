use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut ratios = HashMap::new();
        for rect in rectangles.iter() {
            ratios.entry((rect[0] as f64 / rect[1] as f64).to_ne_bytes()).and_modify(|r| *r += 1).or_insert(1);
        }
        println!("{:?}", ratios);
        
        ratios.values().map(|val| val*(val-1)/2).sum()
    }
}

fn main() {
    let input: Vec<Vec<i32>> = vec![vec![4,5], vec![7,8]];
    println!("{:?}", Solution::interchangeable_rectangles(input));
}