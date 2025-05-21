use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

type OptNode = Option<Rc<RefCell<TreeNode>>>;

struct Solution;
impl Solution {
    fn build_bst(nums: &Vec<i32>, left: i32, right: i32) -> OptNode {
        if left > right {
            return None;
        }

        let mid = right - (right - left) / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));

        root.borrow_mut().left = Self::build_bst(nums, left, mid - 1);
        root.borrow_mut().right = Self::build_bst(nums, mid + 1, right);

        Some(root)
    }
    fn sorted_array_to_bst(nums: Vec<i32>) -> OptNode {
        Self::build_bst(&nums, 0, nums.len() as i32 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![-10,-3,0,5,9];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        let result = Solution::sorted_array_to_bst(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1,3];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let result = Solution::sorted_array_to_bst(nums);
        assert_eq!(result, expected);
    }
}