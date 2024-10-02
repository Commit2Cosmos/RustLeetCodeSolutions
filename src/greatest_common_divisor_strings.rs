pub struct Solution;


impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        for i in (1..=str1.len().min(str2.len())).rev() {
            if str1.len() % i == 0 && str2.len() % i == 0 {
                if str1[..i].repeat(str1.len()/i) == str1 && str1[..i].repeat(str2.len()/i) == str2 {
                    return str1[..i].to_string();
                }
            }
        }

        String::from("")
    }
}