use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
        let mut res = vec![];

        for (mut e, v) in equations.into_iter().zip(values.into_iter()) {
            let (b, a) = (e.pop().unwrap(), e.pop().unwrap());
            graph.entry(a.clone()).or_default().insert(b.clone(), v);
            graph.entry(b).or_default().insert(a, 1_f64/v);
        }

        for mut q in queries {
            let (b, a) = (q.pop().unwrap(), q.pop().unwrap());

            let mut stack: Vec<(String, f64)> = vec![(a, 1_f64)];
            let mut visited: HashSet<String> = HashSet::new();
            let mut has_found = false;

            while let Some((s, val)) = stack.pop() {
                if let Some(map) = graph.get(&s) {
                    if let Some(k) = map.get(&b) {
                        res.push(k*val);
                        has_found = true;
                        break;
                    } else {
                        for (k, v) in map.iter() {
                            if visited.contains(k) {
                                continue;
                            }
                            stack.push((k.to_string(), v*val));
                            visited.insert(k.to_string());
                        }
                    }
                }
            }
            if !has_found {
                res.push(-1.0);
            }
        }


        res
    }
}