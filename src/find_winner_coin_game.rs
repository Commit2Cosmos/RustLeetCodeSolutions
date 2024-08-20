pub struct Solution{
}

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let i = x.min(y/4);

        if i%2==0 {
            return String::from("Bob");
        }

        String::from("Alice")
    }
}