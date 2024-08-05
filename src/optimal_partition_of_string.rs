pub struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {

        let mut appeared_once: Vec<bool> = vec![false; 26];
        let mut mask = 0;
        let mut res: i32 = 0;

        for c in s.chars() {
            // if appeared_once[c as usize - 97] {
            //     // let indices: Vec<usize> = appeared_once.iter_mut().enumerate().filter(|(_, &mut val)| val).map(|(index, val)| {
            //     //     *val = false;
            //     //     index
            //     // }).collect();

            //     // let part: String = indices.iter().map(|&i| std::char::from_u32((i + 97) as u32).unwrap()).collect();

            //     appeared_once = vec![false; 26];
            //     res += 1;
            // }
            // appeared_once[c as usize - 97] = true;
            let cur = 1 << (c as u8 - b'a');
            if (mask & cur) == cur {
                res += 1;
                mask = 0;
            }
            println!("{}", mask);
            println!("{}", cur);
            println!("{}", mask^cur);
            println!("----------------");
            mask ^= cur;
        }

        res + 1
    }
}

fn main() {
    let input: String = String::from("aba");
    println!("{:?}", Solution::partition_string(input));
}