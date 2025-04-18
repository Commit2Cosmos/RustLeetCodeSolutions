pub struct Solution;


impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        for i in (0..digits.len()).rev() {
            if let Some(d) = digits.get_mut(i) {
                if *d != 9 {
                    *d += 1;
                    return digits;
                } else {
                    *d = 0;
                }

            }
        }

        digits.insert(0, 1);

        digits
    }
}