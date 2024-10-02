pub struct Solution;


impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = *candies.iter().max().unwrap() - extra_candies;

        candies.into_iter()
            .map(|x| x >= max_candies)
            .collect()
    }
}