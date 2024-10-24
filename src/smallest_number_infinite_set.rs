use::std::collections::BTreeSet;

pub struct SmallestInfiniteSet {
    min_num: i32,
    smaller_than_min: BTreeSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    pub fn new() -> Self {
        Self {
            min_num: 1,
            smaller_than_min: BTreeSet::new(),
        }
    }
    
    pub fn pop_smallest(&mut self) -> i32 {
        self.smaller_than_min.pop_first().unwrap_or_else(|| {
            self.min_num += 1;
            self.min_num - 1
        })
    }
    
    pub fn add_back(&mut self, num: i32) {
        if num == self.min_num - 1 {
            self.min_num -= 1;
            while self.smaller_than_min.last().is_some_and(|x| *x == self.min_num - 1) {
                self.smaller_than_min.pop_last();
                self.min_num -= 1;
            }
        } else if num < self.min_num {
            self.smaller_than_min.insert(num);
        }
    }
}