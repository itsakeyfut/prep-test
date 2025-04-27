use std::rc::Rc;
use std::cell::RefCell;

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

trait TreeNodeSwap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}
impl TreeNodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right)
    }
    fn swap_all(&mut self) {
        self.left.as_mut().map(|node| node.borrow_mut().swap_all());
        self.right.as_mut().map(|node| node.borrow_mut().swap_all());
        self.swap();
    }
}

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
struct Solution;
impl Solution {
    // BFS
    pub fn invert_tree1(root: OptTreeNode) -> OptTreeNode {
        // ノードを保持するためのキューを初期化
        let mut nodes_to_visit: Vec<OptTreeNode> = Vec::new();

        // 最初のノードをキューに追加
        nodes_to_visit.push(root.clone());

        // キューにノードが残っている限り繰り返し処理
        while let Some(node_opt) = nodes_to_visit.pop() {
            // ノードが存在する場合
            if let Some(node) = node_opt {
                // ノードの可変参照を取得
                let mut borrowed_node = node.borrow_mut();
                // 左右の子ノードを一時的に取り出す
                let left_child = borrowed_node.left.take();
                let right_child = borrowed_node.right.take();
                // 左右を入れ替え
                borrowed_node.left = right_child;
                borrowed_node.right = left_child;
                // 左の子ノードが存在すればキューに追加
                if let Some(left_node) = borrowed_node.left.clone() {
                    nodes_to_visit.push(Some(left_node));
                }
                // 右の子ノードが存在すればキューに追加
                if let Some(right_node) = borrowed_node.right.clone() {
                    nodes_to_visit.push(Some(right_node));
                }
            }
        }
        // 最終的に変更されたツリーのルートを返す
        root
    }

    // DFS
    pub fn invert_tree2(root: OptTreeNode) -> OptTreeNode {
        let mut nodes_to_visit: Vec<OptTreeNode> = vec![root.clone()];
        while let Some(node_opt) = nodes_to_visit.pop() {
            if let Some(node) = node_opt {
                // ノードの可変参照を取得
                let mut borrowed_node = node.borrow_mut();
                // 左右の子ノードを一時的に取り出す
                let left_child = borrowed_node.left.take();
                let right_child = borrowed_node.right.take();
                // 左右を入れ替え
                borrowed_node.left = right_child;
                borrowed_node.right = left_child;
                // 右の子ノードが存在すればスタックに追加
                if let Some(right_node) = borrowed_node.right.clone() {
                    nodes_to_visit.push(Some(right_node));
                }
                // 左の子ノードが存在すればスタックに追加
                if let Some(left_node) = borrowed_node.left.clone() {
                    nodes_to_visit.push(Some(left_node));
                }
            }
        }
        root
    }

    // Iterative and Recursive swapping
    pub fn invert_tree3(root: OptTreeNode) -> OptTreeNode {
        root.map(|node| {
            node.borrow_mut().swap_all();
            node
        })
    }
}

fn main() {
    /*
     * BFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })))
    })));
    println!("After inverting: {:?}", Solution::invert_tree1(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 42,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    println!("After inverting: {:?}", Solution::invert_tree1(root));

    let root = None;
    println!("After inverting: {:?}", Solution::invert_tree1(root));

    /*
     * DFS
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })))
    })));
    println!("After inverting: {:?}", Solution::invert_tree2(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 42,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    println!("After inverting: {:?}", Solution::invert_tree2(root));

    let root = None;
    println!("After inverting: {:?}", Solution::invert_tree2(root));

    /*
     * Iterative and Recursive swapping
     */
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })))
    })));
    println!("After inverting: {:?}", Solution::invert_tree3(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 42,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    println!("After inverting: {:?}", Solution::invert_tree3(root));

    let root = None;
    println!("After inverting: {:?}", Solution::invert_tree3(root));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invert_tree1() {
        // BFS (幅優先探索)
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            })))
        })));
        
        let inverted = Solution::invert_tree1(root.clone());
        assert_eq!(inverted, Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })))
        }))));
    }

    #[test]
    fn test_invert_tree2() {
        // DFS (深さ優先探索)
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            })))
        })));
        
        let inverted = Solution::invert_tree2(root.clone());
        assert_eq!(inverted, Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })))
        }))));
    }

    #[test]
    fn test_invert_tree3() {
        // Iterative and Recursive swapping
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            })))
        })));
        
        let inverted = Solution::invert_tree3(root.clone());
        assert_eq!(inverted, Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })))
        }))));
    }

    #[test]
    fn test_invert_tree_with_none() {
        let root = None;
        let inverted = Solution::invert_tree1(root);
        assert_eq!(inverted, None);
    }
}
