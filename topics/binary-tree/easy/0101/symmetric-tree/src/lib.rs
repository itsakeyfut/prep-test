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
    pub fn is_symmetric(root: OptNode) -> bool {
        fn are_subtrees_symmetric(left_subtree: OptNode, right_subtree: OptNode) -> bool {
            match (left_subtree, right_subtree) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(left_node), Some(right_node)) => {
                    let left = left_node.borrow();
                    let right = right_node.borrow();

                    left.val == right.val
                        && are_subtrees_symmetric(left.left.clone(), right.right.clone())
                        && are_subtrees_symmetric(left.right.clone(), right.left.clone())
                }
            }
        }

        match root {
            Some(root_node) => {
                let root_borrow = root_node.borrow();
                are_subtrees_symmetric(root_borrow.left.clone(), root_borrow.right.clone())
            }
            None => true,
        }
    }
}