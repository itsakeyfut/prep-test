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
    fn cmp(p: &OptNode, q: &OptNode) -> bool {
        match (
            p.as_ref().map(|n| n.borrow()),
            q.as_ref().map(|n| n.borrow())
        ) {
            (Some(p), Some(q)) => {
                p.val == q.val
                    && Self::cmp(&p.left, &q.left)
                    && Self::cmp(&p.right, &q.right)
            },
            (None, None) => true,
            _ => false,
        }
    }
    pub fn is_same_tree1(p: OptNode, q: OptNode) -> bool {
        Self::cmp(&p, &q)
    }

    pub fn is_same_tree2(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree2(p.left.clone(), q.left.clone())
                    && Self::is_same_tree2(p.right.clone(), q.right.clone())
            }
            _ => false
        }
    }

    pub fn is_same_tree3(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}

fn main() {
    /*
     * DFS 1
     */
    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree1(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree1(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
    })));
    println!("{}", Solution::is_same_tree1(p_root, q_root));

    /*
     * DFS 2
     */
    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree2(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree2(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
    })));
    println!("{}", Solution::is_same_tree2(p_root, q_root));

    /*
     * Eq
     */
    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree3(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
    })));
    println!("{}", Solution::is_same_tree3(p_root, q_root));

    let p_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
    })));
    let q_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
    })));
    println!("{}", Solution::is_same_tree3(p_root, q_root));
}
