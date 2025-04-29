# Tail Recursion (末尾再帰)

## ✅ 末尾再帰とは？

関数の**最後の処理が「自分自身の呼び出し」**になっている再帰のこと。

### 普通の再帰（非末尾再帰）

```rs
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 }
    else { n * factorial(n - 1) } // ここで n と掛け算してから戻ってくる必要がある
}
```

### 末尾再帰（Tail Recursion）

```rs
fn factorial_tail(n: u32, acc: u32) -> u32 {
    if n == 0 { acc }
    else { factorial_tial(n - 1, acc * n) } // 最後が再帰呼び出しのみ
}

fn main() {
    println!("{}", factorial_tail(5, 1)); // 120
}
```

末尾再帰の利点：

- 最適化可能
  - ただし Rust は自動でやってくれない
- 明示的にスタックを節約した設計にできる
  - `loop` に書き換えやすい

## 🤔 Rust は最適化しないってどういうこと？

- C や Scala の一部コンパイラは Tail Call Optimization (TCO) を行う
- Rust は TCO を保証しない
  - 再帰が深い場合、loop + 明示的なスタック操作で置き換える野が推奨される

## 🔁 Rust で再帰をループに書き換える

```rs
fn factorial_loop(mut n: u32) -> u32 {
    let mut acc = 1;
    while n > 0 {
        acc *= n;
        n -= 1;
    }
    acc
}
```

ループに変換することで、安全に深い計算ができる。

## ✅ 実用ではどうする？

- 再帰はまず考える（設計がシンプル）
- 深い再帰 or パフォーマンスが重要 → ループに書き換え or スタックを手動管理

## 🧪 演習

次の再帰を末尾再帰化してください：

```rs
fn sum(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + sum(n - 1)
    }
}
```

末尾再帰版（`sum_tail(n, acc)`） を Rust で書いてみましょう！
