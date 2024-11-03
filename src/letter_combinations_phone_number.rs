use lazy_static::lazy_static;

pub struct Solution;

lazy_static! {
    static ref LETTERS: Vec<Vec<char>> = vec![
        ['a', 'b', 'c'].to_vec(),
        ['d', 'e', 'f'].to_vec(),
        ['g', 'h', 'i'].to_vec(), 
        ['j', 'k', 'l'].to_vec(),
        ['m', 'n', 'o'].to_vec(), 
        ['p', 'q', 'r', 's'].to_vec(),
        ['t', 'u', 'v'].to_vec(),
        ['w', 'x', 'y', 'z'].to_vec(),
    ];
}



impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let digits: Vec<usize> = digits.bytes().map(|x| (x - b'2') as usize).collect();

        let mut res = vec![];

        fn recur(idx: usize, curr: String, res: &mut Vec<String>, digits: &Vec<usize>) {
            if idx >= digits.len() {
                res.push(curr.clone());
                return;
            }

            for l in LETTERS[digits[idx]].iter() {
                recur(idx+1, format!("{}{}", curr, *l), res, digits);
            }
        }

        recur(0, String::new(), &mut res, &digits);
       
        res
    }
}