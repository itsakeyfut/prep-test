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
    pub fn count_nodes(root: OptNode) -> i32 {
        fn height(node: &OptNode, go_left: bool) -> i32 {
            let mut h = 0;
            let mut curr = node.clone();
            while let Some(n) = curr {
                h += 1;
                curr = if go_left {
                    n.borrow().left.clone()
                } else {
                    n.borrow().right.clone()
                };
            }
            h
        }

        match root {
            None => 0,
            Some(ref node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                let hl = height(&left, true);
                let hr = height(&right, false);

                if hl == hr {
                    // 完全二分木の場合：ノード数 = 2^(h+1) - 1
                    (1 << (hl + 1)) - 1
                } else {
                    1 + Self::count_nodes(left) + Self::count_nodes(right)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn example1() {
        // ノード作成
        let n1 = node(1);
        let n2 = node(2);
        let n3 = node(3);
        let n4 = node(4);
        let n5 = node(5);
        let n6 = node(6);

        // ツリー構築（完全二分木）
        //         1
        //       /   \
        //      2     3
        //     / \   /
        //    4   5 6
        n1.borrow_mut().left = Some(Rc::clone(&n2));
        n1.borrow_mut().right = Some(Rc::clone(&n3));
        n2.borrow_mut().left = Some(Rc::clone(&n4));
        n2.borrow_mut().right = Some(Rc::clone(&n5));
        n3.borrow_mut().left = Some(Rc::clone(&n6));

        let root = Some(n1);
        let expected = 6;

        assert_eq!(Solution::count_nodes(root), expected);
    }

    #[test]
    fn example2() {
        let root = Option::None;
        let expected = 0;

        assert_eq!(Solution::count_nodes(root), expected);
    }

    #[test]
    fn example3() {
        let n1 = node(1);
        let root = Some(n1);
        let expected = 1;

        assert_eq!(Solution::count_nodes(root), expected);
    }
}
