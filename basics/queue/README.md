# Queue

## VecDeque

`VecDeque<T>` は **「両端キュー（Double-Ended Queue）」**の略です。

- 普通の `Vec` と違って、前（front）と後ろ（back）の両方から要素の追加・削除が高速にできます。
- 内部的にはリングバッファ（循環バッファ）として実装されています。

```rs
use std::collections::VecDeque;
```

## 🔧 よく使うメソッド

| メソッド          | 説明                                     |
| ----------------- | ---------------------------------------- |
| `push_back(val)`  | 後ろに要素を追加（キューの enqueue）     |
| `pop_front()`     | 前から要素を取り出す（キューの dequeue） |
| `push_front(val)` | 前に要素を追加（スタックのように）       |
| `pop_back()`      | 後ろから要素を取り出す                   |

## なぜ VecDeque？

普通の `Vec` だと `pop_front()`（先頭の要素を取り除く）には O(n) のコストがかかります（全体をシフトするため）。
しかし、`VecDeque` は 前後の両端操作が O(1) で済むので、キューのような操作には理想的なんです。

## pop の返り値

```rs
let maybe_val = queue.pop_front();
```

## なぜ Option で返すの？

`pop` は「空だったら何も返せない」ので、Rust ではエラーやヌルポインタの代わりに `Option` を使ってこう表現します：

- `Some(value)` → 要素があったときの値
- `None` → 空だった（値がなかった）

## 典型的な使い方

### 1. if let Some(val) = ...

```rs
if let Some(val) = queue.pop_front() {
    println!("取り出した値: {}", val);
}
```

これは「値があるときだけ処理したい」というケースに便利。

### 2. while let Some(val) = ...

```rs
while let Some(val) = queue.pop_front() {
    println!("処理中: {}", val);
}
```

これは、キューやスタックを全部空になるまで処理したいときにとてもよく使われます。

### 3. match

```rs
match queue.pop_front() {
    Some(val) => println!("値あり{}", val),
    None => println!("空です"),
}
```

こちらは条件分岐を明示的に書きたい場合に使います。

## ✅ Rust のポイント

- ⚠️ ヌル（null）や例外（例外的に落ちる）を使わない。
- ✅ 代わりに Option や Result で「存在しない可能性」を安全に処理する。
