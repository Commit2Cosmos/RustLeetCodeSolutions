mod find_number_winning_players;


fn main() {
    let inp: Vec<(i32, Vec<Vec<i32>>)> = [
        (4, [[0,0],[1,0],[1,0],[2,1],[2,1],[2,0]].iter().map(|&arr| arr.to_vec()).collect()),
        (5, [[1,1],[1,2],[1,3],[1,4]].iter().map(|&arr| arr.to_vec()).collect()),
        (5, [[1,1],[2,4],[2,4],[2,4]].iter().map(|&arr| arr.to_vec()).collect()),
    ].to_vec();


    for (idx, i) in inp.iter().enumerate() {
        let res = find_number_winning_players::Solution::winning_player_count(
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