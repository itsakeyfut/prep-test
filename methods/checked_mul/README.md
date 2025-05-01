# checked_mul

Rust の checked_mul は、オーバーフローのチェックを行う安全な掛け算です。

具体的には、2 つの数値の積がその型の最大値を越える場合、 `None` を返してパニックを回避します。
逆に、オーバーフローしない場合は、結果を `Some(result)` として返します。

## 例

```rs
fn main() {
    let a: u32 = 1_000_000;
    let b: u32 = 5_000;

    match a.checked_mul(b) {
        Some(result) => println!("Result: {}", result),
        None => println!("オーバーフローが発生しました")
    }
}
```

## 解説

- `a * b` は `u32` の範囲（最大値は `4_294_967_295`）を超える可能性があります。
- `checked_mul` は使うと、オーバーフローしても安全に `None` になります。
