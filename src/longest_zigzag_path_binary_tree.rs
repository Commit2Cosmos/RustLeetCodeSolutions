#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
use std::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //* node, count, is_right
        let mut res = 0;
        let mut stack = Vec::new();

        if let Some(root) = root {
            let mut node = root.borrow_mut();
            stack.push((node.left.take(), 0, false));
            stack.push((node.right.take(), 0, true));
        } else {
            return 0;
        }
        
        while let Some(tuple) = stack.pop() {
            if let (Some(node), count, is_right) = tuple {
                let mut node = node.borrow_mut();

                
                let count = if is_right {(count+1, 0)} else {(0, count+1)};
                res = max(res, max(count.0, count.1));

                stack.push((node.left.take(), count.0, false));
                stack.push((node.right.take(), count.1, true));
            }
        }

        
        res
    }
}