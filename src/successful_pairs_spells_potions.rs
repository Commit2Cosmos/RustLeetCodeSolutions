pub struct Solution;


impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions: Vec<i64> = potions.into_iter().map(|x| (success + x as i64 - 1) / x as i64).collect();
        potions.sort_unstable();
        let len = spells.len();
        let mut res: Vec<i32> = Vec::with_capacity(len);

        // println!("{:?}", potions);

        for (_, x) in spells.into_iter().enumerate() {
            let x = x as i64;
            let mut left = 0;
            let mut right = potions.len();

            if x > potions[right-1] {
                res.push(right as i32);
                continue;
            }

            let mut first_mid: Option<usize> = None;
            let mut largest_less_than_target: Option<usize> = None;
            
            while left < right {
                let mid = left + (right-left) / 2;
                if x == potions[mid] {
                    first_mid = Some(mid);
                    left = mid + 1;
                } else if potions[mid] < x {
                    left = mid + 1;
                } else {
                    right = mid;
                    largest_less_than_target = Some(mid);
                }
            }
            // println!("-----------------");
            // println!("{:?}", first_mid);
            // println!("{:?}", largest_less_than_target);

            let first_mid = first_mid.map(|x| x+1).or(largest_less_than_target).unwrap_or(0);
            res.push(first_mid as i32);
        }

        res
    }
}