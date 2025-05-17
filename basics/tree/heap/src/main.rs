#[derive(Debug)]
struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left_child(i: usize) -> usize {
        2 * i + 1
    }

    fn right_child(i: usize) -> usize {
        2 * i + 2
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let min = self.data[0];
        let last = self.data.pop().unwrap();

        if !self.data.is_empty() {
            self.data[0] = last;
            self.heapify_down(0);
        }

        Some(min)
    }

    fn peek(&self) -> Option<i32> {
        self.data.get(0).copied()
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = MinHeap::parent(index);
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = MinHeap::left_child(index);
            let right = MinHeap::right_child(index);

            let mut smallest = index;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }

            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.data.swap(index, smallest);
            index = smallest;
        }
    }
}

fn main() {
    let mut heap = MinHeap::new();

    heap.push(10);
    heap.push(4);
    heap.push(15);
    heap.push(1);

    println!("{:?}", heap.peek()); // Some(1)

    while let Some(val) = heap.pop() {
        println!("{}", val); // 1, 4, 10, 15 の順に出力
    }
}
