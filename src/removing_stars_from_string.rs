pub struct Solution;


impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();

        s.bytes().for_each(|x| {
            if x == b'*' {
                stack.pop();
            } else {
                stack.push(x);
            }
        });

        stack.into_iter().map(|x| char::from(x)).collect()
    }
}