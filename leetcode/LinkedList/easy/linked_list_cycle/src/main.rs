use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, Clone)]
struct ListNode {
    #[allow(unused)]
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

struct Solution {}

impl Solution {
    pub fn has_cycle1(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut visited = HashSet::new();
        let mut curr = head;

        while let Some(node) = curr {
            // Rc<RefCell<ListNode>> の生ポインタを取得
            let ptr = node.as_ptr();

            if visited.contains(&ptr) {
                // もし既に訪問済みなら循環あり
                return true;
            }
            // 訪問済みに追加
            visited.insert(ptr);
            // 次のノードへ
            curr = node.borrow().next.clone();
        }
        false
    }

    /// フロイドの循環検出アルゴリズム (トータス&ヘアー法)
    /// 2つのポインタ (スロー、ファスト) を用意する
    /// slowは1ステップずつ移動
    /// fastは2ステップずつ移動
    /// slowとfastが同じノードに到達したら循環あり
    /// fastがNoneに到達したら循環なし
    pub fn has_cycle2(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while let (Some(slow_node), Some(fast_node)) = (slow.clone(), fast.clone()) {
            if let Some(next_fast) = &fast_node.borrow().next {
                // fast を2ステップ進める
                fast = next_fast.borrow().next.clone();
            } else {
                // fast が None になったら循環なし
                return false;
            }

            // slow を1ステップ進める
            slow = slow_node.borrow().next.clone();

            if let (Some(slow_ref), Some(fast_ref)) = (&slow, &fast) {
                if Rc::ptr_eq(slow_ref, fast_ref) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    /*
     * HashSet
     */
    // 各ノードを作成 (Rc<RefCell<T>> を用いて可変参照を可能にする)
    let node4 = Rc::new(RefCell::new(ListNode { val: -4, next: None }));
    let node3 = Rc::new(RefCell::new(ListNode { val: 0, next: Some(Rc::clone(&node4)) }));
    let node2 = Rc::new(RefCell::new(ListNode { val: 2, next: Some(Rc::clone(&node3)) }));
    let node1 = Rc::new(RefCell::new(ListNode { val: 3, next: Some(Rc::clone(&node2)) }));

    // 循環を作成
    node4.borrow_mut().next = Some(Rc::clone(&node2));

    // リストの head を設定
    let head = Some(node1);
    println!("{}", Solution::has_cycle1(head));

    /*
     * フロイドの循環検出アルゴリズム
     */
    let node4 = Rc::new(RefCell::new(ListNode { val: -4, next: None }));
    let node3 = Rc::new(RefCell::new(ListNode { val: 0, next: Some(Rc::clone(&node4)) }));
    let node2 = Rc::new(RefCell::new(ListNode { val: 2, next: Some(Rc::clone(&node3)) }));
    let node1 = Rc::new(RefCell::new(ListNode { val: 3, next: Some(Rc::clone(&node2)) }));

    node4.borrow_mut().next = Some(Rc::clone(&node2));

    let head = Some(node1);
    println!("{}", Solution::has_cycle2(head));
}
