pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {

        let mut position: (i32, i32) = (0, 0);

        moves.chars().for_each(|m| {
            match m {
                'U' => {position.1 += 1},
                'D' => {position.1 -= 1},
                'L' => {position.0 -= 1},
                'R' => {position.0 += 1},
                _ => unreachable!()
            }
        });

        position == (0, 0)
    }
}

fn main() {
    let moves: String = "UD".to_string();
    println!("{:?}", Solution::judge_circle(moves));
}