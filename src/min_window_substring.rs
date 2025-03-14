pub struct Solution;


impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut map = [0; 128];
        let mut count = t.len();
        let mut start = 0;
        let mut min_start = 0;
        let mut min_len = usize::MAX;

        for &c in t_bytes {
            map[c as usize] += 1;
        }

        for end in 0..s.len() {
            if map[s_bytes[end] as usize] > 0 {
                count -= 1;
            }
            map[s_bytes[end] as usize] -= 1;

            while count == 0 {
                if end - start + 1 < min_len {
                    min_start = start;
                    min_len = end - start + 1;
                }

                map[s_bytes[start] as usize] += 1;
                if map[s_bytes[start] as usize] > 0 {
                    count += 1;
                }
                start += 1;
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            s[min_start..min_start + min_len].to_string()
        }
    }
}