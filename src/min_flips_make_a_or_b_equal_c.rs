pub struct Solution;


impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {

        fn bits(n: i32) -> i32 {
            let n = n as i64;
            let u_count = n - ((n >> 1) & 0o33333333333) - ((n >> 2) & 0o11111111111);
            (((u_count + (u_count >> 3)) & 0o30707070707) % 63) as i32
        }

        let diff = (a | b) ^ c;

        bits(diff) + bits(a & b & diff)
    }
}