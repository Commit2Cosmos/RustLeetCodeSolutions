pub struct Solution;

/** 
    * Forward declaration of guess API.
    * @param  num   your guess
    * @return 	     -1 if num is higher than the picked number
    *			      1 if num is lower than the picked number
    *               otherwise return 0
    * unsafe fn guess(num: i32) -> i32 {}
*/

impl Solution {
    unsafe fn guessNumber(mut n: i32) -> i32 {
        let mut left = 1;
        loop {
            let mut mid = left + (n-left)/2;
            match guess(mid) {
                1 => {
                    left = mid+1;
                },
                -1 => {
                    n = mid-1;
                },
                0 => return mid,
                _ => unreachable!(),
            }
        }
    }
}

unsafe fn guess(num: i32) -> i32 {1}