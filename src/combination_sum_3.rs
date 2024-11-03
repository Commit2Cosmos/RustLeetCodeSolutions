pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {

        let mut res = Vec::new();
        
        fn recur(idx: i32, curr: Vec<i32>, n: i32, k: i32, res: &mut Vec<Vec<i32>>) {
            if k == 0 {
                if n == 0 {
                    res.push(curr.clone());
                } 
                return;
            }
            if n < 0 {
                return;
            }

            for i in (idx + 1)..=9 {
                recur(i, [curr.as_slice(), &[i]].concat(), n-i, k-1, res);
            }
        }
        
        recur(0, Vec::new(), n, k, &mut res);

        res
    }
}