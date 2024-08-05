mod shortest_distance_addition_queries_two;


fn main() {
    let inp: Vec<(i32, Vec<Vec<i32>>)> = [
        (5, [[2,4],[0,2],[0,4]].iter().map(|&arr| arr.to_vec()).collect()),
        (4, [[0,3],[0,2]].iter().map(|&arr| arr.to_vec()).collect()),
    ].to_vec();


    for (idx, i) in inp.iter().enumerate() {
        let res = shortest_distance_addition_queries_two::Solution::shortest_distance_after_queries(
            i.0, i.1.clone()
        );
        println!("Case {}: {:?}", idx, res);
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