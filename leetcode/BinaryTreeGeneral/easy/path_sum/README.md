# Path Sum

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
    // map_or
    pub fn has_path_sum(root: OptNode, target_sum: i32) -> bool {
        // root が None の場合は、パスが存在しないため false を返す
        root.map_or(false, |root| {
            // ノードを借用して内容を参照
            match &*root.borrow() {
                // 葉ノード (左右に子がない) の場合、その値が target_sum と等しいか確認
                TreeNode { val, left: None, right: None } => *val == target_sum,
                // 子ノードがある場合、左右の部分木に対して再帰的に探索
                TreeNode { val, ref left, ref right } => {
                    // 現在のノードの値を差し引いた残りのターゲット合計
                    let remaining_sum = target_sum - val;
                    Self::has_path_sum(left.clone(), remaining_sum) || Self::has_path_sum(right.clone(), remaining_sum)
                }
            }
        })
    }


    // DFS
    pub fn has_path_sum_dfs(root: OptNode, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut stack = vec![(root, target_sum)];

        while let Some((node, remaining_sum)) = stack.pop() {
            if let Some(rc_node) = node {
                let node_ref = rc_node.borrow();
                let new_sum = remaining_sum - node_ref.val;

                // 葉ノードで、かつ target_sum に到達している場合
                if node_ref.left.is_none() && node_ref.right.is_none() && new_sum == 0 {
                    return true;
                }

                // 右子ノードがある場合、スタックに追加
                if let Some(right) = node_ref.right.clone() {
                    stack.push((Some(right), new_sum));
                }
                // 左子ノードがある場合、スタックに追加
                if let Some(left) = node_ref.left.clone() {
                    stack.push((Some(left), new_sum));
                }
            }
        }
        false
    }

    // BFS
    pub fn has_path_sum_bfs(root: OptNode, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root, target_sum));

        while let Some((node, remaining_sum)) = queue.pop_front() {
            if let Some(rc_node) = node {
                let node_ref = rc_node.borrow();
                let new_sum = remaining_sum - node_ref.val;

                // 葉ノードで target_sum に到達している場合
                if node_ref.left.is_none() && node_ref.right.is_none() && new_sum == 0 {
                    return true;
                }

                // 左子ノードがある場合、キューに追加
                if let Some(left) = node_ref.left.clone() {
                    queue.push_back((Some(left), new_sum));
                }
                // 右子ノードがある場合、キューに追加
                if let Some(right) = node_ref.right.clone() {
                    queue.push_back((Some(right), new_sum));
                }
            }
        }
        false
    }
}

fn main() {
    /*
     * map_or
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
            })))
        })))
    })));
    let target_sum = 22;
    println!("{}", Solution::has_path_sum(root, target_sum));

    /*
     * DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
            })))
        })))
    })));
    let target_sum = 22;
    println!("{}", Solution::has_path_sum_dfs(root, target_sum));

    /*
     * BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
            })))
        })))
    })));
    let target_sum = 22;
    println!("{}", Solution::has_path_sum_bfs(root, target_sum));
}
```

```bash
true
true
true
```

### `&*root` について

root は以下のような構造を持っています：

- `Option` は、`Some` か `None` のいずれかを保持します。
- `Rc<RefCell<T>>` は、共有可能な参照カウント付きの可変参照を提供する型です。`TreeNode` はその中身です。

したがって、`root` は「`TreeNode` を参照する可変なポインタ」を包んでいる `Option` 型の値であることになります。

### `&*root`の仕組み

- `*root`
  - `root` は`Option<Rc<RefCell<TreeNode>>>` 型で、最初に参照されているのは `Option` 型です。
    `*` を使って `Option` をアンラップ (解放) し、内部の `Rc<RefCell<TreeNode>>` を取り出します。これで `Rc<RefCell<TreeNode>>` 型が得られます。
- `&*root`
  - `*root` で `Rc<RefCell<TreeNode>>` が得られた後、`&` を使ってその参照を取得します。これで `&Rc<RefCell<TreeNode>>` 型になります。

この操作の目的は、`root` から `Rc<RefCell<TreeNode>>` を取り出し、その参照を `match` 式で使うためです。
`match` 内では、`TreeNode` の内容を取り出すために、その参照が必要です。
