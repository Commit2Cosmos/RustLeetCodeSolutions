use std::collections::{HashMap, VecDeque};

pub struct Solution {
}

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_lst: HashMap<i32, Vec<i32>> = HashMap::new();
        
        //* Create adjacency list */
        for e in edges {
            adj_lst.entry(e[0]).or_insert_with(|| Vec::new()).push(e[1]);
            adj_lst.entry(e[1]).or_insert_with(|| Vec::new()).push(e[0]);
        }

        
        //* Find a & b (nodes with the largest distance between them) */

        //* Find a -> start with random node and find the furthest node */

        // Because it's an acyclic graph, can keep track of prev, instead of having a "visited" set
        // current, previous, distance to initial
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(0,-1, 0)]);
        // farthest node, distance to farthest node
        let mut farthest_a: (i32, i32) = (0, 0);

        while let Some((current, prev, dist_to_parent)) = queue.pop_front() {
            if dist_to_parent > farthest_a.1 {
                farthest_a = (current, dist_to_parent);
            }

            if let Some(neighbour) = adj_lst.get(&current) {
                for &n in neighbour {
                    if n == prev {
                        continue;
                    }
                    if n % 2 == 0 {
                        queue.push_back((n, current, dist_to_parent+2));
                    } else {
                        queue.push_back((n, current, dist_to_parent+1));
                    }
                }
            }
        }

        //* Find b -> start with a and find the furthest node + find distances from a to all other nodes */

        // current, previous, distance to initial
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(farthest_a.0, -1, 0)]);
        // farthest node, distance to farthest node
        let mut farthest_b: (i32, i32) = (farthest_a.0, 0);

        // distance from a to every other node
        let mut dist_from_a = vec![0; adj_lst.len()];

        while let Some((current, prev, dist_to_parent)) = queue.pop_front() {
            if dist_to_parent > farthest_b.1 {
                farthest_b = (current, dist_to_parent);
            }

            if let Some(neighbour) = adj_lst.get(&current) {
                for &n in neighbour {
                    if n == prev {
                        continue;
                    }
                    if n % 2 == 0 {
                        queue.push_back((n, current, dist_to_parent+2));
                        dist_from_a[n as usize] = dist_to_parent+2;
                    } else {
                        queue.push_back((n, current, dist_to_parent+1));
                        dist_from_a[n as usize] = dist_to_parent+1;
                    }
                }
            }
        }

        //* Find distances from b to all other nodes */
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(farthest_b.0, -1, 0)]);

        // distance from b to every other node
        let mut dist_from_b = vec![0; adj_lst.len()];

        while let Some((current, prev, dist_to_parent)) = queue.pop_front() {

            if let Some(neighbour) = adj_lst.get(&current) {
                for &n in neighbour {
                    if n == prev {
                        continue;
                    }
                    if n % 2 == 0 {
                        queue.push_back((n, current, dist_to_parent+2));
                        dist_from_b[n as usize] = dist_to_parent+2;
                    } else {
                        queue.push_back((n, current, dist_to_parent+1));
                        dist_from_b[n as usize] = dist_to_parent+1;
                    }
                }
            }
        }

        let mut res: Vec<i32> = Vec::new();
        
        for i in 0..adj_lst.len() {
            let dist_to_a = dist_from_a[i] - farthest_a.0%2 + i as i32%2;
            let dist_to_b = dist_from_b[i] - farthest_b.0%2 + i as i32%2;
            res.push(dist_to_a.max(dist_to_b));
        }

        res
    }
}