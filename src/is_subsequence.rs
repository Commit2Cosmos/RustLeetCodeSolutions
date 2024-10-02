pub struct Solution;


impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() { return false; }
        if s.is_empty() { return true; }

        let s = s.into_bytes();
        let mut i = 0;

        for x in t.into_bytes() {
            if x == s[i] { 
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }

        false
    }
}