use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
}

struct InOrderIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    current: Option<Rc<RefCell<TreeNode>>>,
}

impl InOrderIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        InOrderIterator {
            stack: vec![],
            current: root,
        }
    }
}

impl Iterator for InOrderIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current.take() {
            self.stack.push(node.clone());
            self.current = node.borrow().left.clone();
        }

        if let Some(node) = self.stack.pop() {
            let val = node.borrow().val;
            self.current = node.borrow().right.clone();
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let root = TreeNode::new(2);
    root.borrow_mut().left = Some(TreeNode::new(1));
    root.borrow_mut().right = Some(TreeNode::new(3));

    let iter = InOrderIterator::new(Some(root));
    for val in iter {
        println!("{}", val);
    }
}

/*
 * 1
 * 2
 * 3
 */