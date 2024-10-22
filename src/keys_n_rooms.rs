struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = Vec::new();
        stack.push(0);
        let mut visited= vec![false; rooms.len()];

        while let Some(k) = stack.pop() {
            for key in rooms[k].iter() {
                if !visited[k] {
                    stack.push(*key as usize);
                }
            }

            visited[k] = true;
        }

        visited.into_iter().all(|x| x)
    }
}