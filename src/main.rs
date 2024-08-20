mod count_number_inversions;


fn main() {
    let inp: Vec<(i32, Vec<Vec<i32>>)> = [
        (3, [[2,2],[0,0]].map(|v| {v.to_vec()}).to_vec())
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = count_number_inversions::Solution::number_of_permutations(
            i.0, i.1
        );
        println!("Case {}: {:?}", idx+1, res);
    }
}