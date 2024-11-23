pub struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for c in connections {
            graph[c[0] as usize].push((c[1] as usize, true));
            graph[c[1] as usize].push((c[0] as usize, false));
        }

        let mut stack = vec![0];
        let mut visited: Vec<bool> = vec![false; n];
        visited[0] = true;
        let mut res = 0;

        while let Some(k) = stack.pop() {
            for &(node, is_away_from_zero) in graph[k].iter() {
                if !visited[node] {
                    if is_away_from_zero {
                        res += 1;
                    }
                    visited[node] = true;
                    stack.push(node);
                }
            }
        }

        res
    }
}