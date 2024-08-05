use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    //*             value, index
    hashmap: HashMap<i32, usize>,
    vector: Vec<i32>
}



impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            hashmap: HashMap::new(),
            vector: Vec::new()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.hashmap.contains_key(&val) {
            return false;
        }
        
        self.hashmap.insert(val, self.vector.len());
        self.vector.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.hashmap.remove(&val) {
                        
            self.vector.swap_remove(index);

            // self.hashmap.remove(&last);
            if self.vector.len() != index {
                println!("now");
                self.hashmap.insert(self.vector[index], index);
            }

            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let rand_num = rand::thread_rng().gen_range(0..self.vector.len());
        self.vector[rand_num]
    }
}






fn main() {
    let mut obj = RandomizedSet::new();
    println!("{}", obj.insert(3));
    println!("{}", obj.insert(3));
    println!("{}", obj.insert(1));
    println!("{:?}", obj.hashmap);
    println!("{:?}", obj.vector);
    println!("{}", obj.remove(3));
}