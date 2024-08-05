// use std::collections::HashSet;

pub struct Solution {
}

impl Solution {
    // pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    //     let mut winners: HashSet<i32> = HashSet::new();
    //     let mut balls = vec![vec![0; 11]; n as usize];

    //     for p in pick {
    //         let (player, colour) = (p[0], p[1]);
    //         if winners.contains(&player) {
    //             continue;
    //         }
    //         balls[player as usize][colour as usize] += 1;
    //         if balls[player as usize][colour as usize] >= player + 1 {
    //             winners.insert(player);
    //         }
    //     }

    //     winners.len() as i32
    // }

    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        pick.into_iter()
        .fold(vec![vec![0; 11]; n as usize], |mut acc, x| {
            acc[x[0] as usize][x[1] as usize] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| {
            if x.into_iter().any(|c| c > i) {
                acc + 1
            } else {
                acc
            }
        })
    }
}