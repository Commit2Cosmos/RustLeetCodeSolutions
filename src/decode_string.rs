pub struct Solution;


impl Solution {

    pub fn decode_string(s: String) -> String {
        //* how many repeat
        let mut stack_times: Vec<usize> = Vec::new();
        //* what to repeat
        let mut stack_chars: Vec<Vec<char>> = Vec::new();

        let mut res = String::new();
        let mut times = String::new();

        s.chars().for_each(|x| {

            match x {
                x if x.is_ascii_digit() => {
                    times.push(x);
                },
                x if x.is_ascii_alphabetic() => {
                    if let Some(l) = stack_chars.last_mut() {
                        l.push(x);
                    } else {
                        res.push(char::from(x));
                    }
                },
                '[' => {
                    stack_chars.push(Vec::new());
                    stack_times.push(times.parse().unwrap());
                    times = String::new();
                },
                ']' => {
                    let to_insert = stack_chars.pop().unwrap().repeat(stack_times.pop().unwrap());
                    if let Some(l) = stack_chars.last_mut() {
                        l.extend(to_insert);
                    } else {
                        res.push_str(&String::from_iter(to_insert));
                    }
                },
                _ => unreachable!()
            }
        });


        res
    }
}