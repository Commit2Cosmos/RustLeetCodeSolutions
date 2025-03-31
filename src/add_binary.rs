pub struct Solution;


impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        
        let (mut i, mut j) = (a.len(), b.len());
        let mut res = String::new();
        let mut carry = 0;
        let mut a = a.chars().rev().peekable();
        let mut b = b.chars().rev().peekable();


        while a.peek().is_some() || b.peek().is_some() || carry > 0 {
            if i > 0 {
                i -= 1;
                carry += a.next().unwrap() as u8 - b'0';
            }
            if j > 0 {
                j -= 1;
                carry += b.next().unwrap() as u8 - b'0';
            }
            res.push((carry%2 + b'0') as char);
            carry /= 2;
        }

        res.chars().rev().collect()
    }
}