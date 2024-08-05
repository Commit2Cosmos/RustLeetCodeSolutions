use std::collections::{ HashMap, VecDeque};

pub struct Solution {
}

impl Solution {

    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {

        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

        let n = n as usize;

        for i in 0..n-1 {
            map.insert(i, vec![i+1]);
        }

        let mut res: Vec<i32> = Vec::new();

        for i in 0..queries.len() {

            let (s, e) = (queries[i][0] as usize, queries[i][1] as usize);

            if let Some(vec) = map.get_mut(&s) {
                vec.push(e);
            }
            
            let mut min_dist: Vec<i32> = vec![10_i32.pow(5); n as usize];
            min_dist[0] = 0;

            let mut q: VecDeque<usize> = VecDeque::new();
            q.push_front(0);

            while q.len() > 0 {
                let cur = q.pop_front().unwrap();

                if let Some(v) = map.get(&cur) {
                    for &k in v {
                        if min_dist[k] > min_dist[cur] + 1 {
                            min_dist[k] = min_dist[cur] + 1;
                            q.push_back(k);
                        }
                    }
                }
            }
            res.push(min_dist[n-1])
        }

        res
    }

}