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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let target_sum = target_sum as i64;
        let mut stack_left: Vec<(Option<Rc<RefCell<TreeNode>>>, Vec<i64>, usize)> = Vec::from([(root, Vec::<i64>::from([target_sum]), 1)]);
        let mut stack_right: Vec<(Option<Rc<RefCell<TreeNode>>>, Vec<i64>, usize)> = Vec::new();
        let mut res: i32 = 0;

        while let Some(tuple) = stack_left.pop().or_else(|| stack_right.pop()) {
            if let (Some(node), mut sum_vec, length) = tuple {
                let mut node = node.borrow_mut();

                while length <= sum_vec.len() {
                    sum_vec.pop();
                }

                sum_vec.push(target_sum);

                for x in sum_vec.iter_mut() {
                    let diff = *x - node.val as i64;
                    if diff == 0 {
                        res += 1;
                    }
                    *x = diff;
                };


                stack_left.push((node.left.take(), sum_vec.clone(), length+1));
                stack_right.push((node.right.take(), sum_vec, length+1));
            }
        }

        res
    }
}