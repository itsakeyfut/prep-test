use std::collections::VecDeque;

struct Queue<T> {
    queue: VecDeque<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, data: T) {
        self.queue.push_back(data);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.front()
    }
}

fn main() {
    let mut queue = Queue::new();

    println!("Peeking a queue before enqueue: {:?}", queue.peek());
    println!("{:?}", queue.queue);
    queue.enqueue(1);
    println!("{:?}", queue.queue);
    queue.enqueue(2);
    println!("{:?}", queue.queue);
    queue.enqueue(3);
    println!("{:?}", queue.queue);
    println!("Peeking a queue after enqueue: {:?}", queue.peek());
    queue.enqueue(4);
    println!("{:?}", queue.queue);

    println!("Peeking a queue after enqueue: {:?}", queue.peek());
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    queue.dequeue();
    println!("{:?}", queue.queue);
    println!("Peeking a queue after dequeue: {:?}", queue.peek());

}