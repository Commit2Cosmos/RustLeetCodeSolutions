mod time_taken_mark_all_nodes;


fn main() {
    let inp: Vec<Vec<Vec<i32>>> = [
        [[0,1],[0,2]].iter().map(|&arr| arr.to_vec()).collect(),
        [[2,4],[0,1],[2,3],[0,2]].iter().map(|&arr| arr.to_vec()).collect(),
        // [[0,1],[0,1],[0,0]].iter().map(|&arr| arr.to_vec()).collect(),
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = time_taken_mark_all_nodes::Solution::time_taken(
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