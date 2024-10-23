pub struct Solution;


impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut visited = vec![false; is_connected.len()];
        
        for i in 0..is_connected.len() {
            if visited[i] {
                continue;
            }
            res += 1;
            
            let mut stack = vec![i];
            
            while let Some(next) = stack.pop() {
                if visited[next] {
                    continue;
                }
                visited[next] = true;
                stack.extend(is_connected[next].iter().enumerate().filter(|&(idx, &x)| !visited[idx] && x == 1).map(|(idx, _)| idx));
            }
        }

        res as i32
    }
}