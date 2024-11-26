use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        const MOVES: [(i32, i32); 4] = [(0,1), (1,0), (0,-1), (-1,0)];

        
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

        let (entr_x, entr_y) = (entrance[0], entrance[1]);
        visited[entr_x as usize][entr_y as usize] = true;
        
        for (i, j) in MOVES {
            let x = i+entr_x;
            let y = j+entr_y;
            if x < 0 || y < 0 || x as usize == maze.len() || y as usize == maze[0].len() || maze[x as usize][y as usize] == '+' {
                continue;
            }
            queue.push_back((x as usize, y as usize, 1));
        }

        while let Some((x, y, steps)) = queue.pop_front() {
            if visited[x][y] || maze[x][y] == '+' {
                continue;
            }
            visited[x][y] = true;

            if x == 0 || x == maze.len()-1 || y == 0 || y == maze[0].len()-1 {
                return steps;
            }

            for (i, j) in MOVES {
                queue.push_back(((i+x as i32) as usize, (j+y as i32) as usize, steps+1));
            }
        }

        return -1;
    }
}