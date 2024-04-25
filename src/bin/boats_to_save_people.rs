pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {

        let mut res: i32 = people.len() as i32;
        let mut left: usize = 0;
        let mut right: usize = people.len()-1;

        people.sort_unstable();

        while left < right {
            if people[left] + people[right] <= limit {
                res -= 1;
                left += 1;
            }
            right -= 1;
        }

        res
    }
}

fn main() {
    let people: Vec<i32> = [1,2].to_vec();
    let limit: i32 = 3;
    println!("{:?}", Solution::num_rescue_boats(people, limit));
}


// diff = 4
// [2]