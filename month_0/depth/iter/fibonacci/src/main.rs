struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let result = Some(self.curr);
        self.curr = self.next;
        self.next = new_next;
        result
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for x in fib.take(10) {
        println!("{x}");
    }

    /*
     * 0
     * 1
     * 1
     * 2
     * 3
     * 5
     * 8
     * 13
     * 21
     * 34
     */
}
