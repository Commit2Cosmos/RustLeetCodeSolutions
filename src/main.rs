mod design_neighbor_sum_service;
use design_neighbor_sum_service::neighborSum;


fn main() {
    // let inp: Vec<&str> = [
    //     "00011",
    //     "101101",
    //     "101"
    // ].to_vec();

    // for (idx, i) in inp.iter().enumerate() {
    //     let res = count_number_of_substrings_with_dominant_ones::Solution::number_of_substrings(i.to_string());
    //     println!("Case {}: {:?}", idx, res);
    // }

    let grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
    .iter()
    .map(|&arr| arr.to_vec())
    .collect();
    
    let obj = neighborSum::new(grid);

    
    let value = 8;

    let ret_1: i32 = obj.adjacent_sum(value);
    let ret_2: i32 = obj.diagonal_sum(value);

    println!("{} {}", ret_1, ret_2)
}