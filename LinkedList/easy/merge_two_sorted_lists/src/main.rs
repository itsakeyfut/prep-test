#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for v in vec {
        let new_node = Some(Box::new(ListNode::new(v)));
        if tail.is_none() {
            head = new_node;
            tail = &mut head;
        } else {
            tail.as_mut().unwrap().next = new_node;
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
    head
}

fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists1(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // list1 の先頭指す参照を 現在のノードに保持
        let mut curr = &mut list1;

        while list2.is_some() {
            // list1 が空、または list2 の現在のノードの値が list1 の現在のノードの値より小さい場合
            if curr.is_none() || list2.as_ref()?.val < curr.as_ref()?.val {
                // list1 の先頭に list2 のノードを移動
                std::mem::swap(curr, &mut list2);
            }
            // 現在のノードを次のノードに進める
            curr = &mut curr.as_mut()?.next;
        }
        // マージされたリストの先頭を返す
        list1
    }

    pub fn merge_two_lists2(
        first_list: Option<Box<ListNode>>,
        second_list: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        match (first_list, second_list) {
            // 両方のリストが空の場合
            (None, None) => None,
            // どちらかが空ならもう一方を返す
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(first_node), Some(second_node)) => {
                // 常に小さい値を優先してリストをマージするため、first_nodeを基準にする
                if first_node.val >= second_node.val {
                    Some(Box::new(ListNode {
                        val: second_node.val,
                        next: Solution::merge_two_lists2(Some(first_node), second_node.next)
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: first_node.val,
                        next: Solution::merge_two_lists2(first_node.next, Some(second_node))
                    }))
                }
            }
        }
    }

    pub fn merge_two_lists3(
        mut first_list: Option<Box<ListNode>>,
        mut second_list: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 片方のリストが空なら、もう片方をそのまま返す
        if first_list.is_none() {
            return second_list;
        }
        if second_list.is_none() {
            return first_list;
        }
        
        // ダミーヘッドノードを作成し、新しいリストの先頭を管理
        let mut dummy_head = ListNode::new(-1);
        let mut curr_tail = &mut dummy_head;

        // 両方のリストが存在する間、値を比較しながらマージ
        while first_list.is_some() && second_list.is_some() {
            if first_list.as_ref()?.val <= second_list.as_ref()?.val {
                curr_tail.next = first_list.take(); // first_list のノードを追加
                curr_tail = curr_tail.next.as_mut()?; // curr_tail を更新
                first_list = curr_tail.next.take(); // first_list を次に進める
            } else {
                curr_tail.next = second_list.take(); // second_list のノードを追加
                curr_tail = curr_tail.next.as_mut()?; // curr_tail を更新
                second_list = curr_tail.next.take(); // second_list を次に進める
            }
        }

        // 残りの要素を追加
        if first_list.is_some() {
            curr_tail.next = first_list.take();
        }
        if second_list.is_some() {
            curr_tail.next = second_list.take();
        }

        // dummy_head の次のノードがマージされたリストの先頭
        dummy_head.next
    }
}

fn main() {
    let list1 = vec_to_list(vec![1, 2, 4]);
    let list2 = vec_to_list(vec![1, 3, 4]);
    let merged_list = Solution::merge_two_lists1(list1, list2);
    let result = list_to_vec(merged_list);
    println!("{:?}", result);

    let list1 = vec_to_list(vec![1, 2, 4]);
    let list2 = vec_to_list(vec![1, 3, 4]);
    let merged_list = Solution::merge_two_lists2(list1, list2);
    let result = list_to_vec(merged_list);
    println!("{:?}", result);

    let list1 = vec_to_list(vec![1, 2, 4]);
    let list2 = vec_to_list(vec![1, 3, 4]);
    let merged_list = Solution::merge_two_lists3(list1, list2);
    let result = list_to_vec(merged_list);
    println!("{:?}", result);
}
