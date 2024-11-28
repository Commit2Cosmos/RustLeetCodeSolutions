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

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from([(root, 1_usize)]);
        let (mut curr_lvl, mut curr_max) = (1, 0);
        let (mut res_lvl, mut res_sum) = (1, i32::MIN);

        while let Some((node, level)) = queue.pop_front() {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                if level == curr_lvl {
                    curr_max += node.val;
                } else {
                    if curr_max > res_sum {
                        res_sum = curr_max;
                        res_lvl = curr_lvl;
                    }
                    curr_lvl = level;
                    curr_max = node.val;
                }
                queue.push_back((node.left.take(), level + 1));
                queue.push_back((node.right.take(), level + 1));
            }
        }

        if curr_max > res_sum {
            res_lvl = curr_lvl;
        }

        res_lvl as i32
    }
}