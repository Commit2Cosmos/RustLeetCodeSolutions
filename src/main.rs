mod queries;



fn main() {
    // let inp = [
    //     [1,2,3,4].to_vec(),
    //     [5,5,5,5].to_vec(),
    //     [0,0,0,0].to_vec()
    // ].to_vec();


    // for (idx, i) in inp.into_iter().enumerate() {
    //     let res = coder_pad::Solution::largest_time_from_digits(
    //         i
    //     );
    //     println!("Case {}: {:?}", idx+1, res);
    // }


    let mut numArray = queries::NumArray::new([9,-8].to_vec());
    numArray.update(0, 3);
    println!("{}", numArray.sum_range(1, 1));
    println!("{}", numArray.sum_range(0, 1));
    numArray.update(1, -3);
    println!("{}", numArray.sum_range(0, 1));
}