pub struct Solution;


impl Solution {
    fn div_ceil(dividend: i32, divisor: i32) -> i32 {
        (dividend + divisor - 1) / divisor
    }

    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut res = 0;
        let mut count: i32 = 1;

        for i in 0..flowerbed.len() {
            if flowerbed[i] == 1 {
                res += (Self::div_ceil(count, 2) - 1).max(0);
                count = 0;
            } else {
                count += 1;
            }
        }

        count += 1;
        res += (Self::div_ceil(count, 2) - 1).max(0);

        res >= n
    }
}