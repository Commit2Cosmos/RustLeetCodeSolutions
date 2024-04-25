#[allow(dead_code)]
struct NumMatrix {
    acc_matrix: Vec<Vec<i32>>
}


impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {

        let cols = matrix[0].len();
        let mut acc_matrix: Vec<Vec<i32>> = vec![vec![0; cols+1]; matrix.len()+1];

        for i in 0..matrix.len() {
            for j in 0..cols {
                acc_matrix[i+1][j+1] = matrix[i][j] + acc_matrix[i][j+1] + acc_matrix[i+1][j] - acc_matrix[i][j];
            }
        }

        // println!("{:?}", acc_matrix);
        

        NumMatrix { 
            acc_matrix
        }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.acc_matrix[row2+1][col2+1] - self.acc_matrix[row1][col2+1] - self.acc_matrix[row2+1][col1] + self.acc_matrix[row1][col1]
    }
}



#[allow(unused)]

fn main() {
    let matrix: Vec<Vec<i32>> = [[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]].map(|x| x.to_vec()).to_vec();
    let obj = NumMatrix::new(matrix);

    let row1 = 1;
    let col1 = 2;
    let row2 = 2;
    let col2 = 4;
    let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);

    println!("{}", ret_1);
}


