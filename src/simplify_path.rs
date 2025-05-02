pub struct Solution;


impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        
        for c in path.split_terminator('/').filter(|&x| x != "") {
            match c {
                ".." => {
                    stack.pop();
                },
                "." => (),
                _ => stack.push(c),
            }
        }

        "/".to_string() + &stack.join("/")
    }
}