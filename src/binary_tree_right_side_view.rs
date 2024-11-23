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

use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        fn recur(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>, level: usize) {
            if let Some(node) = node {

                let mut node = node.borrow_mut();
                if let Some(k) = res.get_mut(level) {
                    *k = node.val;
                } else {
                    res.push(node.val);
                }

                recur(node.left.take(), res, level+1);
                recur(node.right.take(), res, level+1);
            }
        }

        recur(root, &mut res, 0);

        res
    }
}