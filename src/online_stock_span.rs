pub struct StockSpanner {
    //* price, result
    stack: Vec<(i32, usize)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    
    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        
        while let Some(&(val, res)) = self.stack.last() {
            if val <= price {
                span += res;
                self.stack.pop();
            } else {
                break;
            }
        }

        self.stack.push((price, span));
        span as i32
    }
}