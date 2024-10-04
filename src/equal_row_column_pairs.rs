use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut parts_rows: HashMap<Vec<i32>, i32> = HashMap::with_capacity(grid.len());

        grid.iter().for_each(|x| {
            parts_rows.entry(x.to_vec()).and_modify(|e| *e += 1).or_insert(1);
        });

        
        (0..grid.len()).filter_map(|x| {
            let col: Vec<i32> = grid.iter().map(|row| row[x]).collect();
            parts_rows.get(&col)
        }).sum()
    }
}