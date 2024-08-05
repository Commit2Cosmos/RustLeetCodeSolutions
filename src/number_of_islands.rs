use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 { return 0; }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut res: i32 = 0;

        for (row, dummy) in grid.iter().enumerate() {
            for (col, &val) in dummy.iter().enumerate() {
                if (val == '1') && (!visited.contains(&(row, col))) {
                    Self::bfs(row, col, &mut visited, &grid);
                    res += 1;
                }
            }
        }

        res
    }


    fn bfs(row: usize, col: usize, visited: &mut HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) {
        let mut deque: VecDeque<(usize, usize)> = VecDeque::new();

        deque.push_front((row, col));
        visited.insert((row, col));

        const DIRS: [(i32, i32); 4]  = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((r, c)) = deque.pop_front() {
            for (dr, dc) in DIRS {

                let (rdr, cdc) = (r as i32 + dr, c as i32 + dc);

                if (r as i32 + dr >= 0) && (c as i32 + dc >= 0) {

                    let rdr = rdr as usize;
                    let cdc = cdc as usize;

                    if (*grid.get(rdr).and_then(|rr| rr.get(cdc)).unwrap_or(&'0') == '1') && !visited.contains(&(rdr, cdc)) {
                        deque.push_back((rdr, cdc));
                        visited.insert((rdr, cdc));
                    }
                }
            }
        }
    }
}

fn main() {
    let grid: Vec<Vec<char>> = [
        ["1","1","0","0","0"],
        ["1","1","0","0","0"],
        ["0","0","1","0","0"],
        ["0","0","0","1","1"]].map(|x| x.map(|l| l.chars().nth(0).unwrap()).to_vec()).to_vec();

    println!("{:?}", grid);
    println!("{:?}", Solution::num_islands(grid));
}