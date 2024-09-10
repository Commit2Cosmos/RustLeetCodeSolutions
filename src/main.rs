mod subarray_sum_equals_k;



fn main() {
    let inp = [
        ([1,1,1].to_vec(), 2),
        ([1,2,3].to_vec(), 3)
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = subarray_sum_equals_k::Solution::subarray_sum(
            i.0, i.1
        );
        println!("Case {}: {:?}", idx+1, res);
    }


    // let obj = range_sum_query_2d::NumMatrix::new([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]].iter().map(|x| x.to_vec()).collect());
    // let ret_1: i32 = obj.sum_region(2, 1, 4, 3);

    // println!("{:?}", ret_1);
}