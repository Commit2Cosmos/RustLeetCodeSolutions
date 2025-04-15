pub struct Solution;


impl Solution {

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len();
        let cols = board[0].len();

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = 0;
                for m in i.saturating_sub(1)..=(i+1).min(rows-1) {
                    for n in j.saturating_sub(1)..=(j+1).min(cols-1) {
                        if (i, j) != (m, n) {
                            sum += board[m][n] & 1;
                        }
                    }
                }
                if board[i][j] == 1 {
                    if [2, 3].contains(&sum) {
                        board[i][j] = 3;
                    }
                } else {
                    if sum == 3 {
                        board[i][j] = 2;
                    }
                }
            }
        }
        for row in board.iter_mut() {
            for cell in row {
                *cell >>= 1;
            }
        }
    }
}