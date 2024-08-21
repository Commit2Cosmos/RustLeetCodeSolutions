mod two_sum_2_input_array_sorted;


fn main() {
    let inp = [
        ([2,7,11,15].to_vec(), 9),
        ([2,3,4].to_vec(), 6),
        ([-1,0].to_vec(), -1),
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = two_sum_2_input_array_sorted::Solution::two_sum(
            i.0, i.1
        );
        println!("Case {}: {:?}", idx+1, res);
    }
}