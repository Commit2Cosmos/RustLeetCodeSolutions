#[allow(dead_code)]
pub struct NumMatrix {
    prefix: Vec<Vec<i32>>
}


impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut prefix: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()+1]; matrix.len()+1];

        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                prefix[row+1][col+1] = matrix[row][col] + prefix[row+1][col] + prefix[row][col+1] - prefix[row][col];
            }
        }

        Self { prefix }
    }
    
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.prefix[row2 as usize + 1][col2 as usize + 1] - self.prefix[row2 as usize + 1][col1 as usize] - self.prefix[row1 as usize][col2 as usize + 1] + self.prefix[row1 as usize][col1 as usize]
    }
}