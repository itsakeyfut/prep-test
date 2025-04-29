fn main() {
    composite_iter();
    println!("");
    custom_iter_adapter();
    println!("");
    use_twice_iter();
    println!("");
    iter_with_lifetime();
}

/// 合成イテレータ
fn composite_iter() {
    let v = vec![1, 2, 3, 4, 5];

    let sum = v.iter()
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0) // 6 だけフィルタリング
        .sum::<i32>();

    println!("{}", sum); // 6
}

/// 偶数だけ返すイテレータアダプタ
struct Even<I> {
    iter: I,
}

impl<I> Iterator for Even<I>
where
    I: Iterator<Item = i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if x % 2 == 0 {
                return Some(x);
            }
        }
        None
    }
}

// by_ref() を使ってイテレータを二回使う
fn custom_iter_adapter() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let even = Even { iter: v.into_iter() };

    for num in even {
        println!("{}", num);
        /*
         * 2
         * 4
         * 6
         */
    }
}

fn use_twice_iter() {
    let mut v = vec![1, 2, 3, 4, 5];

    let mut iter = v.iter();

    let first_two: Vec<_> = iter.by_ref().take(2).collect();
    let rest: Vec<_> = iter.collect();

    println!("{:?}", first_two); // [1, 2]
    println!("{:?}", rest);      // [3, 4, 5]
}

/// ライフタイム付きイテレータ
struct Lines<'a> {
    text: &'a str,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.text.find('\n') {
            let line = &self.text[..pos];
            self.text = &self.text[pos + 1..];
            Some(line)
        } else if !self.text.is_empty() {
            let line = self.text;
            self.text = "";
            Some(line)
        } else {
            None
        }
    }
}

fn iter_with_lifetime() {
    let text = "hello\nworld\nRust";

    let mut lines = Lines { text };

    while let Some(line) = lines.next() {
        println!("{}", line);
    }

    /*
     * hello
     * world
     * Rust
     */
}