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
    pub fn search_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {

        while let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            match node_ref.val.cmp(&val) {
                std::cmp::Ordering::Less => root = node_ref.right.take(),
                std::cmp::Ordering::Greater => root = node_ref.left.take(),
                std::cmp::Ordering::Equal => return Some(node.to_owned()),
            }
        }

        None
    }
}