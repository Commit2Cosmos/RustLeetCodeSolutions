pub struct Solution {
}


impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut row_count, mut col_count) = (0, 0);
        

        //* loop over rows */
        for i in 0..m {
            row_count += Self::num_flips(&grid[i], n);
        }

        //* loop over cols */
        let mut transposed: Vec<Vec<i32>> = vec![vec![]; n];
        
        for row in grid {
            for (j, val) in row.into_iter().enumerate() {
                transposed[j].push(val);
            }
        }

        for i in 0..n {
            col_count += Self::num_flips(&transposed[i], m);
        }

        row_count.min(col_count)
    }

    fn num_flips(v: &Vec<i32>, size: usize) -> i32 {
        let mut flips = 0;

        for i in 0..(size/2) {
            if v[i] != v[size-1-i] {
                flips += 1;
            }
        }

        flips
    }
}