pub struct Solution;


impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();

        let mut l = 0;
        let mut res = 0;
        let mut extra_steps = 1;

        for r in 1..intervals.len() {
            if intervals[r][0] < intervals[l][1] {
                if intervals[r][1] > intervals[l][1] {
                    extra_steps += 1;
                } else {
                    l += extra_steps;
                    extra_steps = 1;
                }

                res += 1;
            } else {
                l += extra_steps;
                extra_steps = 1;
            }
        }

        res
    }
}