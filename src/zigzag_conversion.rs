pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        if num_rows == 1 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();
        let mut res = String::default();

        let n = 2*(num_rows-1);

        for i in 0..num_rows {
            let mut cols = 0;
            let nn = n - i*2;

            while let Some(&c) = s.get(i+cols*n) {
                res.push(c);
    
                if i != 0 && i != num_rows-1 {
                    if let Some(&k) = s.get(i+nn+cols*n) {
                        res.push(k);
                    }
                }
                cols += 1;
            }
        }

        res
    }
}