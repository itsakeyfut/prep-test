#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, left: None, right: None }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn contains(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            match self.left {
                Some(ref left) => left.contains(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right) => right.contains(value),
                None => false,
            }
        }
    }

    fn find_min(&self) -> i32 {
        match self.left {
            Some(ref left) => left.find_min(),
            None => self.value,
        }
    }

    fn remove(self: Box<Self>, value: i32) -> Option<Box<Node>> {
        if value < self.value {
            Some(Box::new(Node {
                value: self.value,
                left: self.left.and_then(|left| left.remove(value)),
                right: self.right,
            }))
        } else if value > self.value {
            Some(Box::new(Node {
                value: self.value,
                left: self.left,
                right: self.right.and_then(|right| right.remove(value)),
            }))
        } else {
            match (self.left, self.right) {
                (Some(left), Some(right)) => {
                    let min_val = right.find_min();
                    Some(Box::new(Node {
                        value: min_val,
                        left: Some(left),
                        right: right.remove(min_val),
                    }))
                }
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (None, None) => None,
            }
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        print!("{} ", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, value: i32) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    fn contains(&self, value: i32) -> bool {
        match self.root {
            Some(ref node) => node.contains(value),
            None => false,
        }
    }

    fn remove(&mut self, value: i32) {
        self.root = self.root.take().and_then(|node| node.remove(value));
    }

    fn print_in_order(&self) {
        if let Some(ref node) = self.root {
            node.in_order();
            println!();
        } else {
            println!("Tree is empty");
        }
    }
}

fn main() {
    let mut bst = BST::new();
    bst.insert(8);
    bst.insert(3);
    bst.insert(10);
    bst.insert(1);
    bst.insert(6);
    bst.insert(14);
    bst.insert(4);
    bst.insert(7);

    bst.print_in_order(); // 1 3 4 6 7 8 10 14

    println!("Contains 6? {}", bst.contains(6)); // true
    println!("Contains 13? {}", bst.contains(13)); // false

    bst.remove(3);
    bst.print_in_order(); // 1 4 6 7 8 10 14
}
