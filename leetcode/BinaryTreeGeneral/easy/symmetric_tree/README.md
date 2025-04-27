# Symmetric Tree

```rust
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

type OptNode = Option<Rc<RefCell<TreeNode>>>;
struct Solution;
impl Solution {
    pub fn is_symmetric1(root: OptNode) -> bool {
        // 再帰的に左右の部分木を比較する
        fn are_subtrees_symmetric(left_subtree: OptNode, right_subtree: OptNode) -> bool {
            match (left_subtree, right_subtree) {
                // 両方の部分木が存在しない場合、対称
                (None, None) => true,
                // どちらか一方の部分木のみが存在する場合、対称でない
                (None, Some(_)) | (Some(_), None) => false,
                // 両方の部分木が存在する場合
                (Some(left_node), Some(right_node)) => {
                    let left = left_node.borrow();
                    let right = right_node.borrow();

                    // ノードの値が等しく、左部分木の左と右部分木の右
                    // 左部分木の右と右部分木の左が対称であれば全体も対称
                    left.val == right.val
                        && are_subtrees_symmetric(left.left.clone(), right.right.clone())
                        && are_subtrees_symmetric(left.right.clone(), right.left.clone())
                }
            }
        }

        // ルートノードが None の場合は対称とみなす
        match root {
            Some(root_node) => {
                let root_borrow = root_node.borrow();
                are_subtrees_symmetric(root_borrow.left.clone(), root_borrow.right.clone())
            }
            None => true,
        }
    }

    pub fn is_symmetric2(root: OptNode) -> bool {
        if root.is_none() {
            return true;
        }

        let mut stack = vec![];
        let root_node = root.unwrap();
        let root_borrow = root_node.borrow();

        stack.push((root_borrow.left.clone(), root_borrow.right.clone()));

        while let Some((left, right)) = stack.pop() {
            match (left, right) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(left_node), Some(right_node)) => {
                    let left_borrow = left_node.borrow();
                    let right_borrow = right_node.borrow();

                    if left_borrow.val != right_borrow.val {
                        return false;
                    }

                    stack.push((left_borrow.left.clone(), right_borrow.right.clone()));
                    stack.push((left_borrow.right.clone(), right_borrow.left.clone()));
                }
            }
        }
        true
    }

    pub fn is_symmetric3(root: OptNode) -> bool {
        if root.is_none() {
            return true;
        }

        let mut queue = std::collections::VecDeque::new();

        // ルートの左右の子ノードをキューに入れる
        let root_node = root.unwrap();
        let root_borrow = root_node.borrow();
        queue.push_back((root_borrow.left.clone(), root_borrow.right.clone()));

        while let Some((left_node, right_node)) = queue.pop_front() {
            match (left_node, right_node) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(left), Some(right)) => {
                    let left_borrow = left.borrow();
                    let right_borrow = right.borrow();

                    if left_borrow.val != right_borrow.val {
                        return false;
                    }

                    // 左部分木の左 & 右部分の右、左部分の右 & 右部分の左 をキューに追加
                    queue.push_back((left_borrow.left.clone(), right_borrow.right.clone()));
                    queue.push_back((left_borrow.right.clone(), right_borrow.left.clone()));
                }
            }
        }

        true
    }
}

fn main() {
    /*
     * Recursive DFS (大規模には向かない)
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric1(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric1(root));

    /*
     * Iterative DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric2(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric2(root));

    /*
     * Iterative BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric3(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    println!("Symmetric: {}", Solution::is_symmetric3(root));
}
```

```bash
Symmetric: true
Symmetric: false
Symmetric: true
Symmetric: false
Symmetric: true
Symmetric: false
```
