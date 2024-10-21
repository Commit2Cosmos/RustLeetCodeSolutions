// Definition for a binary tree node.
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

struct Solution;

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // let mut v1 = vec![];
        // let mut v2 = vec![];
        // Self::get_leaves(root1.unwrap(), &mut v1);
        // Self::get_leaves(root2.unwrap(), &mut v2);

        // v1 == v2

        Self::leaves(root1) == Self::leaves(root2)
    }

    fn get_leaves(root: Rc<RefCell<TreeNode>>, leafs: &mut Vec<i32>) {

        let mut has_children = false;
        
        if let Some(l) = &root.borrow().left {
            has_children = true;
            Self::get_leaves(l.clone(), leafs);
        }

        if let Some(r) = &root.borrow().right {
            has_children = true;
            Self::get_leaves(r.clone(), leafs);
        }

        if !has_children {
            leafs.push(root.borrow().val);
        }
    }

    fn leaves(root1: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        stack.push(root1);

        let mut res = Vec::new();

        while let Some(node) = stack.pop() {
            let n = node.unwrap();
            let node = n.borrow();


            let r = node.right.clone();
            let l = node.left.clone();

            if !r.is_some() && !l.is_some() { res.push(node.val); continue; }

            if r.is_some() { stack.push(r); } 
            if l.is_some()  { stack.push(l); }
        }

        res
    }
}