pub struct Solution;


impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();

        let mut l = 0;
        let mut res = 1;
        let mut extra_steps = 1;
        let mut min_end = points[0][1];

        for r in 1..points.len() {
            if points[r][0] <= min_end {
                min_end = min_end.min(points[r][1]);
                extra_steps += 1;
            } else {
                l += extra_steps;
                extra_steps = 1;
                min_end = points[l][1];
                res += 1;
            }
        }

        res
    }
}