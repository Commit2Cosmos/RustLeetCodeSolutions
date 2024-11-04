pub struct Solution;


impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let (mut a, mut b, mut c) = (0, 1, 1);
        for _ in 0..n {
            (a, b, c) = (b, c, a+b+c);
        }
        return a;
    }
}