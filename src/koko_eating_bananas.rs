pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {

        let mut right = *piles.iter().max().unwrap();
        let mut left = 1;

        'wh: while right > left {

            let mut tot = 0;
            let mid = (right + left)/2;
            
            for i in piles.iter() {
                
                tot += (i+mid-1) / mid;

                // println!("{}", tot);
                // println!("{}", mid);
                // println!("----------------------------");

                if tot > h {
                    left = mid + 1;
                    continue 'wh;
                }
            }

            right = mid;
        }

        left
    }
}

fn main() {
    let piles: Vec<i32> = [30,11,23,4,20].to_vec();
    let h: i32 = 6;
    println!("{:?}", Solution::min_eating_speed(piles, h));
}