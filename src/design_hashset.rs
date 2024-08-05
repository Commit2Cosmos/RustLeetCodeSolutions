pub struct MyHashSet {

}

//* Check: open addressing, hash function, rehashing, chaining

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        
    }
    
    fn add(&self, key: i32) {
        
    }
    
    fn remove(&self, key: i32) {
        
    }
    
    fn contains(&self, key: i32) -> bool {
        
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */


fn main() {
    let key = 3;
    let obj = MyHashSet::new();
    obj.add(key);
    obj.remove(key);
    let ret_3: bool = obj.contains(key);
}