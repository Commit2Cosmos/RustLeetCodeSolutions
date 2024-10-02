pub struct Solution;


impl Solution {

    fn is_vowel(ch: u8) -> bool {
        matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max_v = s[..k as usize].to_ascii_lowercase().as_bytes().iter().filter(|x| Self::is_vowel(**x)).count();
        let mut res = max_v;
        let mut s_iter = s.as_bytes().iter();

        for x in s.to_ascii_lowercase().as_bytes().iter().skip(k as usize) {
            match (Self::is_vowel(*x), Self::is_vowel(*s_iter.next().unwrap())) {
                (true, false) => {
                    max_v += 1;
                    res = res.max(max_v);
                },
                (false, true) => max_v -= 1,
                _ => ()
            }
        }

        res as i32
    }
}