# BinaryHeap (Priority Queue)

Rust 版の優先度付きキュー

- デフォルトでは最大ヒープ（大きい値から出てくる）

## 特徴

- 最小ヒープにしたいから、`Reverse` を使う（これ超重要）

```rs
use std::cmp::Reverse;
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);

heap.push(Reverse(3));
```
