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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (p, q) = (p.unwrap(), q.unwrap());
        let (p, q) = (p.borrow().val, q.borrow().val);

        fn recur(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = node {
                let mut n = node.borrow_mut();

                if n.val == p || n.val == q {
                    return Some(node.clone());
                }

                let l = recur(n.left.take(), p, q);
                let r = recur(n.right.take(), p, q);

                match (l, r) {
                    (None, Some(k)) => Some(k),
                    (Some(k), None) => Some(k),
                    (Some(_), Some(_)) => Some(node.clone()),
                    (None, None) => None,
                }

            } else {
                None
            }
        }

        recur(root, p, q)
    }
}