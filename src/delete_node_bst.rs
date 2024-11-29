
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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node_ref = node.borrow_mut();
            match node_ref.val.cmp(&key) {
                std::cmp::Ordering::Less => node_ref.right = Self::delete_node(node_ref.right.clone(), key),
                std::cmp::Ordering::Greater => node_ref.left = Self::delete_node(node_ref.left.clone(), key),
                std::cmp::Ordering::Equal => {
                    if node_ref.left.is_none() {
                        return node_ref.right.clone();
                    }
                    if node_ref.right.is_none() {
                        return node_ref.left.clone();
                    }

                    let mut cur = node_ref.right.clone().unwrap();

                    while let Some(left) = cur.clone().borrow().left.clone() {
                        cur = left;
                    }
                    
                    node_ref.val = cur.borrow().val;
                    node_ref.right = Self::delete_node(node_ref.right.clone(), node_ref.val);
                },
            }
        }
        return root;
    }
}