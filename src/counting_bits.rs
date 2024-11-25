pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        fn bits(n: i32) -> i32 {
            let n = n as i64;
            let u_count = n - ((n >> 1) & 0o33333333333) - ((n >> 2) & 0o11111111111);
            (((u_count + (u_count >> 3)) & 0o30707070707) % 63) as i32
        }
        (0..=n).map(|x| bits(x)).collect()
    }
}