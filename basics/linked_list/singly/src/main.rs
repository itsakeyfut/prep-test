#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct SinglyLinkedList {
    head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    /// 先頭に挿入 O(1)
    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// 値を検索 O(n)
    fn contains(&self, target: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == target {
                return true;
            }
            current = &node.next;
        }
        false
    }

    /// 先頭の値を削除 O(1)
    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    /// 最後尾に挿入 O(n)
    fn push_back(&mut self, value: i32) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { value, next: None }));
    }

    /// 値を削除（最初に一致したもののみ、O(n) ）
    fn remove(&mut self, target: i32) -> bool {
        let mut current = &mut self.head;
        while let Some(mut boxed_node) = current.take() {
            if boxed_node.value == target {
                *current = boxed_node.next.take();
                return true;
            } else {
                *current = Some(boxed_node);
                current = &mut current.as_mut().unwrap().next;
            }
        }
        false
    }

    /// 全要素表示
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = SinglyLinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    list.push_back(4);
    list.push_back(5);

    list.print(); // 1 -> 2 -> 3 -> 4 -> 5 -> None

    println!("Contains 3? {}", list.contains(3)); // true
    println!("Pop front: {:?}", list.pop_front()); // Some(1)
    list.print(); // 2 -> 3 -> 4 -> 5 -> None

    list.remove(4);
    list.print(); // 2 -> 3 -> 5 -> None
}
