
pub struct Solution {}


impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {

        if arr.len() == 0 {
            return 0;
        }

        let mut res = 1;

        let mut curr_even = 1;
        let mut curr_odd = 1;

        for (i, n) in (1..arr.len()).enumerate() {

            if i % 2 == 0 {
                if arr[n] > arr[n-1] {
                    curr_even += 1;
                    curr_odd = 1;
                } else if arr[n] < arr[n-1] {
                    curr_odd += 1;
                    curr_even = 1;
                } else {
                    curr_even = 1;
                    curr_odd = 1;
                }
            } else {
                if arr[n] < arr[n-1] {
                    curr_even += 1;
                    curr_odd = 1;
                } else if arr[n] > arr[n-1] {
                    curr_odd += 1;
                    curr_even = 1;
                } else {
                    curr_even = 1;
                    curr_odd = 1;
                }
            }

            res = res.max(curr_even.max(curr_odd))
        }

        res
    }
}