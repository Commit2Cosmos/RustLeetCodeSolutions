pub struct Solution;


fn contains_uniq<'a>(it: impl Iterator<Item = &'a char>) -> bool {
    let mut nums = [false; 10];

    it.filter_map(|x| x.to_digit(10)).all(|x| {
        if nums[x as usize] {
            false
        } else {
            nums[x as usize] = true;
            true
        }
    })
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

        let build_square = |i, j| board[i..i+3].iter().flat_map(move |d| &d[j..j+3]);
        
        //* check rows
        board.iter().all(|arr| {
            contains_uniq(arr.iter())
        })
        &&
        //* check cols
        (0..9).all(|i| contains_uniq(board.iter().map(|r| &r[i])))
        &&
        //* check squares
        (0..3).all(|i| (0..3).all(|j| contains_uniq(build_square(i*3, j*3))))
    }
}