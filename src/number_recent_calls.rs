use std::collections::VecDeque;

#[derive(Default)]
pub struct RecentCounter {
    queue: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    
    pub fn ping(&mut self, mut t: i32) -> i32 {
        self.queue.push_back(t);
        t -= 3000;
        while let Some(&f) = self.queue.front() {
            if f < t  {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}