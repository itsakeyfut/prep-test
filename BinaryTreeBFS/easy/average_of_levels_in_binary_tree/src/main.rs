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
    pub fn average_of_levels_bfs(root: OptNode) -> Vec<f64> {
        let mut node_queue = std::collections::VecDeque::new();

        // ルートノードが存在する場合はキューに追加
        if let Some(root_node) = root {
            node_queue.push_back(root_node);
        }

        // 各レベルの平均を格納するためのベクタ
        let mut level_averages = Vec::new();

        // キューが空でない限りループを続ける
        while !node_queue.is_empty() {
            // 現在のレベルにあるノード数
            let current_level_size = node_queue.len();
            // 現在のレベルのノードの値の合計
            let mut level_sum: i64 = 0;
            // 現在のレベルにあるノードをすべて処理
            for _ in 0..current_level_size {
                if let Some(current_node) = node_queue.pop_front() {
                    // ノードの値を合計に加算
                    level_sum += current_node.borrow().val as i64;
                    // 左の子ノードが存在する場合、キューに追加
                    if let Some(left_child) = current_node.borrow_mut().left.take() {
                        node_queue.push_back(left_child);
                    }
                    // 右の子ノードが存在する場合、キューに追加
                    if let Some(right_child) = current_node.borrow_mut().right.take() {
                        node_queue.push_back(right_child);
                    }
                }
            }
            // 現在のレベルの平均を計算し、結果に返却
            level_averages.push(level_sum as f64 / current_level_size as f64);
        }
        // 各レベルの平均値のベクタを返す
        level_averages
    }

    // DFS
    pub fn average_of_levels_dfs(root: OptNode) -> Vec<f64> {
        let mut sum_per_level = Vec::new();   // 各レベルの合計値
        let mut count_per_level = Vec::new(); // 各レベルのノード数
        
        // DFS のヘルパー関数を定義
        fn dfs(node: &OptNode, depth: usize, sum_per_level: &mut Vec<i64>, count_per_level: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                
                // 現在の階層が初めて出現する場合、新しくベクタに追加
                if sum_per_level.len() <= depth {
                    sum_per_level.push(0);
                    count_per_level.push(0);
                }
                
                // 現在の階層の合計値とノード数を更新
                sum_per_level[depth] += n.val as i64;
                count_per_level[depth] += 1;

                // 左右の子ノードを DFS で探索
                dfs(&n.left, depth + 1, sum_per_level, count_per_level);
                dfs(&n.right, depth + 1, sum_per_level, count_per_level);
            }
        }

        // ルートノードから DFS を開始
        dfs(&root, 0, &mut sum_per_level, &mut count_per_level);

        // 各レベルの平均値を計算
        sum_per_level.into_iter()
            .zip(count_per_level.into_iter())
            .map(|(sum, count)| sum as f64 / count as f64)
            .collect()
    }
}

fn main() {
    /*
     * BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        })))
    })));
    println!("{:?}", Solution::average_of_levels_bfs(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: None,
            right: None
        })))
    })));
    println!("{:?}", Solution::average_of_levels_bfs(root));

    /*
     * DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        })))
    })));
    println!("{:?}", Solution::average_of_levels_dfs(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: None,
            right: None
        })))
    })));
    println!("{:?}", Solution::average_of_levels_dfs(root));
}