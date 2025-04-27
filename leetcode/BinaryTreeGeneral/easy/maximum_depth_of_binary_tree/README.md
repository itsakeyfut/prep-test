# Maximum Depth of Binary Tree

```rust
// Rc は Reference Counted の略で、参照カウントを持つスマートポインタです。これにより、複数の場所から同じノードを参照できるようになります。
use std::rc::Rc;
// RefCell は可変の借用を提供する型で、内部のデータを変更することができます。
use std::cell::RefCell;
use std::cmp;

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

struct Solution {}

impl Solution {
    pub fn max_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                // cmp::max(a, b) は a と b のうち大きい方を返します。
                cmp::max(
                Solution::max_depth1(node.borrow().left.clone()),
                Solution::max_depth1(node.borrow().right.clone())
                ) + 1
            }
            None => 0
        }
    }

    pub fn max_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        // スタックには (ノード、現在の深さ) を格納
        let mut stack = vec![(root, 0)];
        // スタックが空でない間、反復処理
        while let Some((curr_node, curr_depth)) = stack.pop() {
            // ノードが存在する場合
            if let Some(node) = curr_node {
                // ノードを借用して参照を得る
                let node = node.borrow();
                // 現在のノードの深さを1増やす
                let depth = curr_depth + 1;
                // 最大深さを更新
                max_depth = max_depth.max(depth);
                // 左部分木をスタックに追加
                stack.push((node.left.clone(), depth));
                // 右部分木をスタックに追加
                stack.push((node.right.clone(), depth));
            }
        }
        max_depth
    }
}

fn main() {
    // ツリーの構築
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
        }))),
    })));

    println!("Max Depth: {}", Solution::max_depth1(root.clone()));
    println!("Max Depth: {}", Solution::max_depth2(root));
}
```

```bash
Max Depth: 3
Max Depth: 3
```
