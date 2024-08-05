#[allow(non_camel_case_types)]
pub struct neighborSum {
    grid: Vec<Vec<i32>>,
}

#[derive(Clone)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    const fn adjacent() -> &'static [Direction]{
        &[
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: 1, y: 0 },
        ]
    }

    const fn diagonal() -> &'static [Direction]{
        &[
            Direction { x: 1, y: 1 },
            Direction { x: 1, y: -1 },
            Direction { x: -1, y: 1 },
            Direction { x: -1, y: -1 },
        ]
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl neighborSum {

    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        neighborSum { grid }
    }
    

    pub fn adjacent_sum(&self, value: i32) -> i32 {
        let (i, j) = self.find_num(value);

        self.find_sum(i as i32, j as i32, Direction::adjacent().to_vec())
    }
    

    pub fn diagonal_sum(&self, value: i32) -> i32 {
        let (i, j) = self.find_num(value);

        self.find_sum(i as i32, j as i32, Direction::diagonal().to_vec())
    }


    fn find_sum(&self, i: i32, j: i32, dirs: Vec<Direction>) -> i32 {
        let mut sum = 0;
        for d in dirs {
            if (0..self.grid.len() as i32).contains(&(d.x + i)) && (0..self.grid.len() as i32).contains(&(d.y + j)) {
                sum += self.grid[(d.x + i) as usize][(d.y + j) as usize]
            }
        }

        sum
    }


    fn find_num(&self, value: i32) -> (usize, usize) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                if value == self.grid[i][j] {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }
}