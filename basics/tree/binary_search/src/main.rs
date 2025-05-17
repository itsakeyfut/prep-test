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
                    let min_val = right.min();
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

    fn min(&self) -> i32 {
        match self.left {
            Some(ref node) => node.min(),
            None => self.value,
        }
    }

    fn max(&self) -> i32 {
        match self.right {
            Some(ref node) => node.max(),
            None => self.value,
        }
    }

    fn pre_order(&self, result: &mut Vec<i32>) {
        result.push(self.value);
        if let Some(ref left) = self.left {
            left.pre_order(result);
        }
        if let Some(ref right) = self.right {
            right.pre_order(result);
        }
    }

    fn post_order(&self, result: &mut Vec<i32>) {
        if let Some(ref left) = self.left {
            left.post_order(result);
        }
        if let Some(ref right) = self.right {
            right.post_order(result);
        }
        result.push(self.value);
    }

    fn height(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |node| node.height());
        let right_height = self.right.as_ref().map_or(0, |node| node.height());
        1 + left_height.max(right_height)
    }

    fn is_balanced(&self) -> bool {
        fn check(node: &Option<Box<Node>>) -> (bool, i32) {
            if let Some(n) = node {
                let (left_bal, left_height) = check(&n.left);
                let (right_bal, right_height) = check(&n.right);

                let balanced = left_bal && right_bal && (left_height - right_height).abs() <= 1;
                (balanced, 1 + left_height.max(right_height))
            } else {
                (true, 0)
            }
        }

        check(&Some(Box::new(self.clone()))).0
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

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            value: self.value,
            left: self.left.clone(),
            right: self.right.clone(),
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

    fn remove(&mut self, value: i32) {
        self.root = self.root.take().and_then(|node| node.remove(value));
    }

    fn contains(&self, value: i32) -> bool {
        self.root.as_ref().map_or(false, |node| node.contains(value))
    }

    fn min(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.min())
    }

    fn max(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.max())
    }

    fn pre_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            node.pre_order(&mut result);
        }
        result
    }

    fn post_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            node.post_order(&mut result);
        }
        result
    }

    fn height(&self) -> i32 {
        self.root.as_ref().map_or(0, |node| node.height())
    }

    fn is_balanced(&self) -> bool {
        self.root.as_ref().map_or(true, |node| node.is_balanced())
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
    println!("Min: {:?}", bst.min()); // Some(1)
    println!("Max: {:?}", bst.max()); // Some(14)
    println!("Pre-order: {:?}", bst.pre_order()); // [8, 3, 1, 6, 4, 7, 10, 14]
    println!("Post-order: {:?}", bst.post_order()); // [1, 4, 7, 6, 3, 14, 10, 8]
    println!("Height: {}", bst.height()); // 4
    println!("Is Balanced: {}", bst.is_balanced()); // true

    bst.remove(3);
    bst.print_in_order(); // 1 4 6 7 8 10 14
}
