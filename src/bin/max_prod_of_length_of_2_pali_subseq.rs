use std::cmp;

pub struct Solution;


//* rewritten to understand filter_map vs filter + map
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let len = s.len();
        //* vector of tuples [mask, length] of each palindrome
        let palindromes: Vec<(usize, usize)> = (1..(1 << len))
            .map(|mask: usize| {

                let subseq: String = (0..len)
                    //* check which letters are present
                    .filter_map(|x| {
                        if mask & (1 << x) > 0 {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    //* add char to subsequence
                    .map(|x| s.chars().nth(x).unwrap())
                    .collect();

                (mask, subseq)

            })
            .filter(|x| {
                //* check if palindrome
                x.1 == x.1.chars().rev().collect::<String>()
            })
            .map(|x| (x.0, x.1.len()))
            .collect();

        
        
        //* sort palindromes by length in decending order
        // palindromes.sort_by(|a, b| b.1.cmp(&a.1));


        //* loop over to find largest lengths, so that masks are non-overlapping
        let mut res: i32 = 0;

        for i in 0..palindromes.len() {
            for j in i+1..palindromes.len() {
                let (mask1, length1) = palindromes[i];
                let (mask2, length2) = palindromes[j];

                if mask1 & mask2 == 0 {
                    res = cmp::max(res, (length1 * length2) as i32);
                } 
            }
        }
        
        // println!("{:?}", res);

        res
        

    }
}

fn main() {
    let input = String::from("accbcaxxcxx");
    Solution::max_product(input);
}


// impl Solution {
//     pub fn max_product(s: String) -> () {
//         let len = s.len();
//         //* vector of tuples [mask, length] of each palindrome
//         let palindromes: Vec<(usize, usize)> = (1..(1 << len))
//             .filter_map(|mask| {

//                 let subseq: String = (0..len)
//                     //* check which letters are present
//                     .filter(|x| mask & (1 << x) > 0)
//                     //* add char to subsequence
//                     .map(|x| s.chars().nth(x).unwrap())
//                     .collect();

//                 //* check if palindrome
//                 if subseq == subseq.chars().rev().collect::<String>() {
//                     Some((mask, subseq.len()))
//                 } else {
//                     None
//                 }

//             })
//             .collect();

//         println!("{:?}", palindromes)
//     }
// }