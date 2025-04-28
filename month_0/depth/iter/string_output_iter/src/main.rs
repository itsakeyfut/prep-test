struct Lines<'a> {
    remaining: &'a str,
}

impl<'a> Lines<'a> {
    fn new(text: &'a str) -> Self {
        Lines { remaining: text }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        if let Some(pos) = self.remaining.find('\n') {
            let line = &self.remaining[..pos];
            self.remaining = &self.remaining[pos + 1..];
            Some(line)
        } else {
            // 最後の行（改行なし）
            let line = self.remaining;
            self.remaining = "";
            Some(line)
        }
    }
}

fn main() {
    let text = "hello\nworld\nRust\n";
    for line in Lines::new(text) {
        println!("{}", line);
    }
}

/*
 * hello
 * world
 * Rust
 */
