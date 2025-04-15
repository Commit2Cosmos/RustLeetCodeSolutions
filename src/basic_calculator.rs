pub struct Solution;


impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut curr, mut sign, mut res) = (0, 1, 0);

        //* number, sign
        let mut stack: Vec<i32> = Vec::new();

        for c in s.into_bytes() {
            println!("curr: {}, sign: {}, res: {}", curr, sign, res);
            match c {
                b' ' => {},
                b'(' => {
                    stack.push(res);
                    stack.push(sign);
                    res = 0;
                    sign = 1;
                },
                b')' => {
                    res += sign * curr;
                    res *= stack.pop().unwrap();
                    res += stack.pop().unwrap();
                    sign = 1;
                    curr = 0;
                },
                b'+' => {
                    res += sign * curr;
                    sign = 1;
                    curr = 0;
                },
                b'-' => {
                    res += sign * curr;
                    sign = -1;
                    curr = 0;
                },
                _ => {
                    curr *= 10;
                    curr += (c - b'0') as i32;
                }
            }
        }

        res += sign * curr;

        res
    }
}