pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
        let mut res: Vec<Vec<String>> = vec![];

        // println!("{:?}", board);

        //* +ve & -ve diagonals
        // let mut diagonals: (HashSet<usize>, HashSet<isize>) = (HashSet::new(), HashSet::new());

        Self::backtrack(0, &mut board, &mut res, n);

        res
    }

    fn backtrack(row: usize, board: &mut Vec<Vec<char>>, res: &mut Vec<Vec<String>>, n: usize) {
        if row == n {
            //* collect '.' into String "...."
            res.push(board.iter().map(|row| row.iter().collect()).collect());
            return;
        }

        for col in 0..n {
            if Self::is_safe(board, row, col) {
                board[row][col] = 'Q';
                Self::backtrack(row+1, board, res, n);
                board[row][col] = '.';
            }
        }
    }

    fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        //* don't check rows, since moving to next one anyway

        //* is column safe
        if !board.iter().all(|x| x.iter().nth(col).unwrap() == &'.') { return false; }


        let (mut i, mut j) = (row as i8, col as i8);
        
        
        //* is +ve diagonal safe
        while i >= 0 && j < board.len() as i8 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        
        let (mut i, mut j) = (row as i8, col as i8);
        //* is -ve diagonal safe
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        true
    }
}

fn main() {
    let n: i32 = 4;
    println!("{:?}", Solution::solve_n_queens(n));
}