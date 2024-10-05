pub struct Solution;


impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        
        'outer: for x in asteroids {
            if x < 0 {
                while let Some(&k) = stack.last() {
                    if k < 0 {
                        stack.push(x);
                        continue 'outer;
                    }
                    match k.cmp(&x.abs()) {
                        std::cmp::Ordering::Less => {
                            stack.pop();
                        },
                        std::cmp::Ordering::Equal => {
                            stack.pop();
                            continue 'outer;
                        },
                        std::cmp::Ordering::Greater => {
                            continue 'outer;
                        },
                    }
                }
                stack.push(x);
            } else {
                stack.push(x);
            }
        }

        stack
    }
}