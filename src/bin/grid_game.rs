use std::cmp;

pub struct Solution;


#[allow(dead_code)]
impl Solution {
    //* finds path with most points for first, clears visited cells, then finds path with most points for second (greedy) | DOESN'T WORK
    fn grid_game_brute(grid: Vec<Vec<i32>>) -> i64 {
        let mut grid_copy = grid.to_owned();
        let len: usize = grid_copy[0].len();

        
        let (index, _max) = Solution::find_best_path(&grid_copy, len);
        // println!("{}", max);
        Solution::clear_fields(&mut grid_copy, index, len);
        println!("{:?}", grid_copy);

        let (_, max_sum) = Solution::find_best_path(&grid_copy, len);
        
        max_sum
    }

    fn find_best_path(grid: &Vec<Vec<i32>>, len: usize) -> (usize, i64) {
        let mut max_index: usize = 0;
        let mut max_sum: i32 = 0;

        for change_index in 0..len {
            // println!("{}", change_index);
            let mut sum: i32 = 0;
            // println!("{}", change_index);

            for current_index in 0..len {
                if current_index < change_index {
                    sum += grid[0][current_index];
                } else if current_index > change_index {
                    sum += grid[1][current_index];
                } else {
                    sum += grid[0][current_index] + grid[1][current_index];
                }
                // println!("{}", sum);
            }

            if sum > max_sum {
                println!("{max_sum}");
                max_sum = sum;
                max_index = change_index;
            }
        }
        (max_index, max_sum as i64)
    }

    fn clear_fields(grid: &mut Vec<Vec<i32>>, index: usize, len: usize) -> () {
        for current_index in 0..len {
            if current_index <= index {
                grid[0][current_index] = 0;
            } 
            if current_index >= index {
                grid[1][current_index] = 0;
            }
        }
    }


    fn grid_game(grid: Vec<Vec<i32>>) -> i64 {

        let len: usize = grid[0].len();

        let mut top_pre: Vec<i64> = vec![0; len+1];
        let mut bot_pre: Vec<i64> = vec![0; len+1];

        //* fill two prefix vectors
        for i in 0..len {
            top_pre[i+1] = top_pre[i] + grid[0][i] as i64;
            bot_pre[i+1] = bot_pre[i] + grid[1][i] as i64;
        }

        // println!("{:?}", top_pre);
        // println!("{:?}", bot_pre);

        let mut res = i64::max_value();

        for i in 1..len+1 {
            let collected_top = top_pre.last().unwrap() - top_pre[i];
            let collected_bot = bot_pre[i-1];
            let collected_second = cmp::max(collected_bot, collected_top);
            res = cmp::min(res, collected_second as i64);
        }

        res
    }

}


fn main() {
    let input = vec![vec![2,5,4], vec![1,5,1]];
    println!("{:?}", Solution::grid_game(input));
}