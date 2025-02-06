# Average of Levels in Binary Tree

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
            let mut level_sum = 0;
            // 現在のレベルにあるノードをすべて処理
            for _ in 0..current_level_size {
                if let Some(current_node) = node_queue.pop_front() {
                    // ノードの値を合計に加算
                    level_sum += current_node.borrow().val;
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
}
```

```bash
[3.0, 14.5, 11.0]
```
