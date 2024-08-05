
pub struct Solution;

#[derive(Debug)]

pub enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

impl Solution {
    pub fn solution(position: (i32, i32), direction: Direction) -> i32 {

        let (x, y) = position;

        match direction {
            Direction::U => Self::how_many(y.is_positive(), x == 0),
            Direction::D => Self::how_many(y.is_negative(), x == 0),
            Direction::L => Self::how_many(x.is_negative(), y == 0),
            Direction::R => Self::how_many(x.is_positive(), y == 0),
        }
    }

    fn how_many(in_same_dir: bool, on_0: bool) -> i32 {
        if in_same_dir { return 2; }
        else if on_0 {
            return 0;
        }
        1
    }
}


fn main() {
    #[cfg(test)]
    mod tests {

        use crate::{Solution, Direction};

        #[test]
        fn it_works() {
            assert_eq!(Solution::solution((0, 0), Direction::U), 0);
            assert_eq!(Solution::solution((1, 0), Direction::L), 0);
            assert_eq!(Solution::solution((1, 1), Direction::R), 2);
            assert_eq!(Solution::solution((-2, 3), Direction::D), 1);
            assert_eq!(Solution::solution((2,-4), Direction::R), 2);
        }
    }
}


        //*     0   , 0   ->    U -> 0, D -> 0, L -> 0, R -> 0
        
        //*     +ve   , 0 ->    U -> 1, D -> 1, L -> 0, R -> 2
        //*     -ve   , 0 ->    U -> 1, D -> 1, L -> 2, R -> 0
        
        //*     0   , +ve ->    U -> 2, D -> 0, L -> 1, R -> 1
        //*     +ve , +ve ->    U -> 2, D -> 1, L -> 1, R -> 2
        //*     -ve , -ve ->    U -> 1, D -> 2, L -> 2, R -> 1
        
        //*     0   , -ve ->    U -> 0, D -> 2, L -> 1, R -> 1
        //*     -ve , +ve ->    U -> 2, D -> 1, L -> 2, R -> 1
        //*     +ve , -ve ->    U -> 1, D -> 2, L -> 1, R -> 2