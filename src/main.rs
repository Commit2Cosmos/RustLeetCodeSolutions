mod contains_duplicate_2;


fn main() {
    let inp = [
        ([1,2,3,1].to_vec(), 3),
        ([1,0,1,1].to_vec(), 1),
        ([1,2,3,1,2,3].to_vec(), 2)
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = contains_duplicate_2::Solution::contains_nearby_duplicate(
            i.0, i.1
        );
        println!("Case {}: {:?}", idx+1, res);
    }
}