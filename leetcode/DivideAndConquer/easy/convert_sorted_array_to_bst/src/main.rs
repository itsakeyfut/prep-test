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
    fn build_bst(nums: &Vec<i32>, left: i32, right: i32) -> OptNode {
        if left > right {
            return None;
        }

        let mid = (left + right) / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));

        // 左右の部分木を構築
        root.borrow_mut().left = Self::build_bst(nums, left, mid - 1);
        root.borrow_mut().right = Self::build_bst(nums, mid + 1, right);

        Some(root)
    }
    // DFS
    pub fn sorted_array_to_bst_dfs(nums: Vec<i32>) -> OptNode {
        Self::build_bst(&nums, 0, nums.len() as i32 -1)
    }

    // BFS
    pub fn sorted_array_to_bst_bfs(nums: Vec<i32>) -> OptNode {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((Rc::clone(&root), 0, nums.len() - 1));

        while let Some((node, left, right)) = queue.pop_front() {
            let mid = (left + right) / 2;

            // 左部分木の作成
            if left < mid {
                let left_mid = (left + mid - 1) / 2;
                let left_node = Rc::new(RefCell::new(TreeNode::new(nums[left_mid])));
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push_back((left_node, left, mid - 1));
            }

            //  右部分木の作成
            if mid < right {
                let right_mid = (mid + 1 + right) / 2;
                let right_node = Rc::new(RefCell::new(TreeNode::new(nums[right_mid])));
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push_back((right_node, mid + 1, right));
            }
        }
        Some(root)
    }
}

fn main() {
    /*
     * DFS
     * 配列の中央をルートにすることで、高さバランスを維持
     * 再帰で左右の部分木を構築 (分割統治法：Divide & Conquer)
     * 計算量 O(n), メモリ O(log n) 再帰スタック
     */
    let nums = vec![-10, -3, 0, 5, 9];
    println!("{:?}", Solution::sorted_array_to_bst_dfs(nums));

    let nums = vec![1, 3];
    println!("{:?}", Solution::sorted_array_to_bst_dfs(nums));

    /*
     * BFS
     * VecDeque を使ってレベルごとにノートを追加・処理
     * キューで (ノード、左端、右端) を管理し、バランスを保つ
     * 計算量 O(n), メモリ O(n) キューにノードを格納
     */

    let nums = vec![-10, -3, 0, 5, 9];
    println!("{:?}", Solution::sorted_array_to_bst_bfs(nums));

    let nums = vec![1, 3];
    println!("{:?}", Solution::sorted_array_to_bst_bfs(nums));
}