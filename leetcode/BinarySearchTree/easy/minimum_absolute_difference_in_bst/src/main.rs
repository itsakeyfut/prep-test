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
    fn inorder_traversal(node: &OptNode, sorted_values: &mut Vec<i32>) {
        if let Some(current_node) = node {
            let node = current_node.borrow();
            // 左部分木を探索
            Self::inorder_traversal(&node.left, sorted_values);
            // 現在のノードの値を記録
            sorted_values.push(node.val);
            // 右部分木を探索
            Self::inorder_traversal(&node.right, sorted_values);
        }
    }
    // DFS
    // O(n)
    pub fn get_minimum_difference_dfs(root: OptNode) -> i32 {
        let mut sorted_values = Vec::new();
        // BST を中序走査し、ノードの値を昇順に取得
        Self::inorder_traversal(&root, &mut sorted_values);

        let mut min_diff = i32::MAX;
        // 隣接するノードの値の差を計算し、最小値を求める
        for i in 1..sorted_values.len() {
            min_diff = min_diff.min(sorted_values[i] - sorted_values[i - 1]);
        }
        min_diff
    }

    // BFS
    // O(n log n) ソートが必要なため
    pub fn get_minimum_difference_bfs(root: OptNode) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let mut values = Vec::new();

        if let Some(n) = root {
            queue.push_back(n);
        }

        // 全ノードの探索
        while let Some(node) = queue.pop_front() {
            let node_ref = node.borrow();
            values.push(node_ref.val);

            if let Some(left) = node_ref.left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push_back(right);
            }
        }
        // 取得したノードの値を昇順にソート
        values.sort();
        // 最小絶対値を求める
        let mut min_diff = i32::MAX;
        for i in 1..values.len() {
            min_diff = min_diff.min(values[i] - values[i - 1]);
        }

        min_diff
    }
}

fn main() {
    /*
     * DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
    })));
    println!("{}", Solution::get_minimum_difference_dfs(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 48,
            left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(49)))),
        })))
    })));
    println!("{}", Solution::get_minimum_difference_dfs(root));

    /*
     * BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
    })));
    println!("{}", Solution::get_minimum_difference_bfs(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 48,
            left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(49)))),
        })))
    })));
    println!("{}", Solution::get_minimum_difference_bfs(root));
}