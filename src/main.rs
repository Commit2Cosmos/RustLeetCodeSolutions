mod min_flips_grid_palindromic;


fn main() {
    let inp: Vec<Vec<Vec<i32>>> = [
        [[1,0,0],[0,0,0],[0,0,1]].iter().map(|&arr| arr.to_vec()).collect(),
        [[0,1],[0,1],[0,0]].iter().map(|&arr| arr.to_vec()).collect(),
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = min_flips_grid_palindromic::Solution::min_flips(
            i
        );
        println!("Case {}: {:?}", idx+1, res);
    }


    // let grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
    // .iter()
    // .map(|&arr| arr.to_vec())
    // .collect();
    
    // let obj = neighborSum::new(grid);

    
    // let value = 8;

    // let ret_1: i32 = obj.adjacent_sum(value);
    // let ret_2: i32 = obj.diagonal_sum(value);

    // println!("{} {}", ret_1, ret_2)
}