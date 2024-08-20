pub struct Solution{
}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        // 2*n/3
        let mut letters: Vec<i32> = vec![0; 26];
        let mut res = s.len() as i32;

        for l in s.into_bytes() {
            letters[l as usize-97] += 1;
        }

        println!("{:?}", letters);

        for l in letters {
            if l == 0 {
                continue;
            }
            if l%2==1 {
                res -= l-1;
                continue;
            }
            res -= l-2;
        }

        res
    }
}