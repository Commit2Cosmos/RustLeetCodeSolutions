mod longest_turbulent_subarray;


fn main() {
    let inp: Vec<Vec<i32>> = [
        [9,4,2,10,7,8,8,1,9].to_vec(),
        [4,8,12,16].to_vec(),
        [100].to_vec(),
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = longest_turbulent_subarray::Solution::max_turbulence_size(
            i
        );
        println!("Case {}: {:?}", idx+1, res);
    }
}