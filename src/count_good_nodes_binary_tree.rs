use std::rc::Rc;
use std::cell::RefCell;

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

pub struct Solution;

impl Solution {

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, i32::MIN)];
        let mut res = 0;

        while let Some(tuple) = stack.pop() {
            if let (Some(node), mut min_val) = tuple {
                let mut node = node.borrow_mut();

                if node.val >= min_val {
                    res += 1;
                    min_val = node.val;
                }

                stack.push((node.left.take(), min_val));
                stack.push((node.right.take(), min_val));
            }
        }

        res
    }


    pub fn good_nodes_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {        
        fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>, min_val: i32) -> i32 {
            if root.is_none() {
                return 0;
            }

            let rc = root.unwrap();

            let v = rc.borrow().val;

            if v >= min_val {
                return 1 + dfs_recur(rc.borrow().left.clone(), v) + dfs_recur(rc.borrow().right.clone(), v);
            } else {
                return dfs_recur(rc.borrow().left.clone(), min_val) + dfs_recur(rc.borrow().right.clone(), min_val);
            }
        }

        dfs_recur(root, i32::MIN)
    }
}