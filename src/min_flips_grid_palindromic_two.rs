pub struct Solution {
}

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut res = 0;
        //* (1,0) */
        let mut single_ones = 0;
        //* (1,1) */
        let mut double_ones = 0;

        for i in 0..m/2 {

            //* change 0 -> 1 or 1 -> 0, whichever is less costly */
            for j in 0..n/2 {
                let to_change = grid[i][j] + grid[i][n-1-j] + grid[m-1-i][j] + grid[m-1-i][n-1-j];
                res += to_change.min(4-to_change);
            }

            if n%2 == 1 {
                let top_bot_sum = grid[i][n/2] + grid[m-1-i][n/2];

                if top_bot_sum == 1 {
                    single_ones += 1;
                } else if top_bot_sum == 2 {
                    double_ones += 1;
                }
            }
        }

        if m%2 == 1 {
            for i in 0..n/2 {
                let left_right_sum = grid[m/2][i] + grid[m/2][n-1-i];

                if left_right_sum == 1 {
                    single_ones += 1;
                } else if left_right_sum == 2 {
                    double_ones += 1;
                }
            }

            //* center cell */
            if n%2 == 1 && grid[m/2][n/2] == 1 {
                res += 1;
            }
        }

        //* if double_ones are even or (double_ones are odd and single_ones > 0) */
        if (double_ones % 2 == 0) || (single_ones > 0) {
            return res + single_ones;
        }

        //* switch the one of the (1,1) pair to (0,0) to make their numbers even */
        res + 2
    }
}