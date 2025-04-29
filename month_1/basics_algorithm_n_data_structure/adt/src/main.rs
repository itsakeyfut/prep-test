// Stack という ADT を定義
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val)
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&i32> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    println!("Top: {:?}", stack.peek()); // Some(20)
    println!("Pop: {:?}", stack.pop()); // Some(20)
    println!("Is Empty: {}", stack.is_empty()); // false
}
