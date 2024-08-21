mod longest_rep_char_replacement;


fn main() {
    let inp = [
        ("ABAB".to_string(), 2),
        ("AABABBA".to_string(), 1),
        ("ABAA".to_string(), 0)
    ].to_vec();


    for (idx, i) in inp.into_iter().enumerate() {
        let res = longest_rep_char_replacement::Solution::character_replacement(
            i.0, i.1
        );
        println!("Case {}: {:?}", idx+1, res);
    }
}