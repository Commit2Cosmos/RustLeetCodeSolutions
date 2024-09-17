pub struct Solution {}

impl Solution {
    
    fn permute(arr: &mut Vec<i32>, start: usize, results: &mut Vec<Vec<i32>>) {
        if start == arr.len() {
            results.push(arr.clone());
        } else {
            for i in start..arr.len() {
                arr.swap(start, i);
                Solution::permute(arr, start + 1, results);
                arr.swap(start, i);
            }
        }
    }

    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut results = Vec::new();
        let mut arr_copy = arr.clone();
    
        Solution::permute(&mut arr_copy, 0, &mut results);
        let mut max_time = -1;

        for p in results {
            let hours = p[0] * 10 + p[1];
            let mins = p[2] * 10 + p[3];

            if hours < 24 && mins < 60 {
                max_time = max_time.max(hours * 60 + mins);
            }
        }
        
        if max_time >= 0 {
            let h = max_time / 60;
            let m = max_time % 60;
            format!("{:02}:{:02}", h, m)
        } else {
            "".to_string()
        }

    }
}