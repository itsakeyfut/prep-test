use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    val: i32,
    prev: Option<Weak<RefCell<Node>>>, // 弱参照（循環参照防止）
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    // 先頭に追加 O(1)
    fn push_front(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev= Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                // 空リストの場合はtailも更新
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    // 末尾に追加 O(1)
    fn push_back(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            prev: self.tail.as_ref().map(|tail| Rc::downgrade(tail)),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                // 空リストの場合
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // 任意の値の後ろに挿入（最初に見つけた target の次に新しい値を入れる） O(n)
    fn insert_after(&mut self, target: i32, val: i32) -> bool {
        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().val == target {
                let next = node.borrow().next.clone();
                let new_node = Rc::new(RefCell::new(Node {
                    val,
                    prev: Some(Rc::downgrade(&node)),
                    next,
                }));

                // 現ノードの次を new_node に更新
                node.borrow_mut().next = Some(new_node.clone());

                // 次のノードがあれば prev を new_node に更新
                if let Some(next_node) = new_node.borrow().next.clone() {
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                } else {
                    // next が None なら tail を更新
                    self.tail = Some(new_node);
                }
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    // 最初に一致する値のノードを削除 O(n)
    fn remove(&mut self, target: i32) -> bool {
        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().val == target {
                let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());
                let next = node.borrow().next.clone();

                match (prev, next) {
                    (Some(prev_node), Some(next_node)) => {
                        // 中間のノードを削除する場合
                        prev_node.borrow_mut().next = Some(next_node.clone());
                        next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
                    }
                    (Some(prev_node), None) => {
                        // tail を削除する場合
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node);
                    }
                    (None, Some(next_node)) => {
                        // head を削除する場合
                        next_node.borrow_mut().prev = None;
                        self.head = Some(next_node);
                    }
                    (None, None) => {
                        // リストにノードが1津田家の場合
                        self.head = None;
                        self.tail = None;
                    }
                }
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    // 値を検索 O(n)
    fn contains(&self, target: i32) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().val == target {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    // 先頭から表示
    fn print_forward(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().val);
            current = node.borrow().next.clone();
        }
        println!("None");
    }

    // 末尾から表示
    fn print_backward(&self) {
        let mut current = self.tail.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().val);
            current = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        }
        println!("None");
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_front(20);
    list.push_front(10);
    list.push_back(30);
    list.print_forward(); // 10 -> 20 -> 30 -> None
    list.print_backward(); // 30 -> 20 -> 10 -> None

    list.insert_after(20, 25); 
    list.print_forward(); // 10 -> 20 -> 25 -> 30 -> None

    println!("Contains 25? {}", list.contains(25)); // true
    println!("Contains 40? {}", list.contains(40)); // false

    list.remove(20);
    list.print_forward(); // 10 -> 25 -> 30 -> None

    list.remove(10);
    list.print_forward(); // 25 -> 30 -> None

    list.remove(30);
    list.print_forward(); // 25 -> None
}
