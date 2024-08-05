pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {

        let mut res: Vec<Vec<i32>> = vec![];

        Self::backtrack(n, k, 1, &mut vec![], &mut res);

        res
    }


    fn backtrack(n: i32, k: i32, start: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k == 0 {
            res.push(comb.clone());
            return;
        }

        for i in start..=n-k+1 {
            comb.push(i);
            Self::backtrack(n, k-1, i+1, comb, res);
            comb.pop();
        }
    }
}

fn main() {
    let n: i32 = 4;
    let k: i32 = 2;
    println!("{:?}", Solution::combine(n, k));
}