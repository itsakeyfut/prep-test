use std::{cell::RefCell, rc::Rc};


type OptNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: OptNode,
    pub right: OptNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn max_depth(root: OptNode) -> i32 {
        match root {
            Some(node) => {
                std::cmp::max(
                    Solution::max_depth(node.borrow().left.clone()),
                    Solution::max_depth(node.borrow().right.clone())
                ) + 1
            }
            None => 0
        }
    }
}