pub struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let l = s.len();
        let ch = ['0','1'];
        let mut diff_count = 0;

        for i in 0..l {
            if s[i] != ch[i%2] {
                diff_count += 1;
            }
        }

        let mut res = diff_count.min(l - diff_count);

        for i in 0..l {
            if s[i] != ch[i%2] {
                diff_count -= 1;
            }

            if s[i] != ch[(i+l)%2] {
                diff_count += 1;
            }

            res = res.min(diff_count.min(l - diff_count));
        }

        res as i32
    }
}

fn main() {
    let s: String = "10001100101000000".to_string();
    println!("{:?}", Solution::min_flips(s));
}