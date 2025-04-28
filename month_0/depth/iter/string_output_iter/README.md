# 文字列を行単位で出すイテレータ

## 仕様

- 長い文字列（&str）を渡すと
- 1 行ずつ順番に出てくるイテレータを作る

例えば：

```rs
let next = "hello\nworld\nRust\n";
for line in Lines::new(text) {
    println!("{}", line);
}
// => hello
//    world
//    Rust
```

## 設計の考え方

- `&str` は「スライス（借用）」だから、データをコピーせず slice を切り出すのが Rust 流
- `lines()` という標準のヘルパーもあるけど、自作して鍛える！
- ライフタイム `'a` が絶対に必要

## ✨ コード

```rs
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
```

## 🧠 ポイント解説

- remaining に「まだ読んでない部分」を持ってる
- find('\n') で次の改行を探す
- slice [start..end] を切り出す（コピーなし！）
- &'a str だから借用ルールを破らない
- 最後の行だけは改行がない場合もあるので注意
