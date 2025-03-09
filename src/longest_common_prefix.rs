pub struct Solution;


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter().reduce(|acc, cur| {
            acc.chars()
            .zip(cur.chars())
            .map_while(|(a, c)| if a == c { Some(a) } else { None })
            .collect()
        }).unwrap()
    }
}