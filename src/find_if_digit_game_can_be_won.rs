
pub struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut singles: i32 = 0;
        let mut doubles: i32 = 0;

        for i in nums {
            if i > 9 {
                doubles += i;
            } else {
                singles += i;
            }
        }

        singles != doubles
    }
}