use std::collections::VecDeque;

fn reverse(queue: &mut VecDeque<i32>) {
    let mut new_queue = VecDeque::new();

    while let Some(val) = queue.pop_front() {
        new_queue.push_front(val);
    }

    *queue = new_queue;
}

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    queue.push_back(4);

    println!("Original queue: {:?}", queue);

    reverse(&mut queue);

    println!("Reversed queue: {:?}", queue);
}
