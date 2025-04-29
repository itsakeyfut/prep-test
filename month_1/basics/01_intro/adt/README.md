# ADT (Abstract Data Type)

## ✍️ 設計視点まとめ

**ADT（抽象データ型）**とは

「中身を隠して、使い方（操作）だけを定義した型のこと」

つまり

- 内部構造は見せない
- 提供する操作（インターフェース）だけを意識して使う

という考え方

### 例

| ADT 名      | できる操作（インターフェース）        |
| :---------- | :------------------------------------ |
| Stack       | push(), pop(), peek(), is_empty()     |
| Queue       | enqueue(), dequeue(), is_empty()      |
| Linked List | insert(), delete(), traverse()        |
| Tree        | insert(), search(), traverse()        |
| Graph       | add_edge(), remove_edge(), traverse() |

- これらは中が配列だろうとリストだろうと、
  使う側には関係ないことが大事！

## 🧠 なぜこれが重要か？

- 「使う側は中身を知らなくていい」
- 「作る側はどんなふうに中身を最適化してもいい」
- Rust の世界でも、ライブラリ設計やプログラム分割でめちゃくちゃ大事になる考え方

> Rust で言えば、Struct と Impl で包み込むことが「ADT を作る」ということ

```rs
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
```

### ✅ ポイント

- Vec を使って実装しているが、使う側は Vec を実装しない（これが ADT）
- もしあとで `LinkedList` に変えたくなっても、インターフェースは変えなくていいい

## ✅ ここまでで理解すべきこと

- ADT とは「中身を隠して使い方だけを見せる」こと
- Rust では Struct + Impl で ADT を自然に作る
- 使う側は、中身（Vec か何か）なんて気にしないでいい
