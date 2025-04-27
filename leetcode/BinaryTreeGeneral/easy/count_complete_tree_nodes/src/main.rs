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
    pub fn count_nodes(root: OptNode) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let mut node_ref = node.borrow_mut();
                1 + Self::count_nodes(node_ref.left.take()) + Self::count_nodes(node_ref.right.take())
            }
        }
    }

    // DFS
    pub fn count_nodes_dfs(root: OptNode) -> i32 {
        // root が None ならノード数は0
        if root.is_none() {
            return 0;
        }

        // root が Some の場合は、左と右のサブツリーを再帰的にカウントして、1を加算 (現在のノードをカウント)
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        1 + Self::count_nodes_dfs(left) + Self::count_nodes_dfs(right)
    }

    // BFS
    pub fn count_nodes_bfs(root: OptNode) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut cnt = 0;
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(root);

        while let Some(node) = queue.pop_front() {
            if let Some(node_ref) = node {
                cnt += 1;

                // 左と右の子ノードをキューに追加
                let left = node_ref.borrow().left.clone();
                let right = node_ref.borrow().right.clone();

                if left.is_some() {
                    queue.push_back(left);
                }
                if right.is_some() {
                    queue.push_back(right);
                }
            }
        }
        cnt
    }
}

fn main() {
    /*
     * Recursive
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: None
        })))
    })));
    println!("{}", Solution::count_nodes(root));

    /*
     * DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: None
        })))
    })));
    println!("{}", Solution::count_nodes_dfs(root));

    /*
     * BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: None
        })))
    })));
    println!("{}", Solution::count_nodes_bfs(root));
}