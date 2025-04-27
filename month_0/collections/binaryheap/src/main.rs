fn main() {
    let mut heap = std::collections::BinaryHeap::new();

    heap.push(3);
    heap.push(1);
    heap.push(5);

    while let Some(top) = heap.pop() {
        println!("{}", top); // 5, 3, 1の順で取り出す
    }
}
