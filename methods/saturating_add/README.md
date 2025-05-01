# .saturating_add()

## 特徴：

- オーバーフローする加算
  - 型の最大値に固定（例：`u32::MAX`）

## 例：

```rs
fn main() {
    let a: u32 = u32::MAX;
    let b: u32 = 1;

    let result = a.saturating_add(b);

    println!("{}", result); // 4294967295
}
```

ここでは `a + b = 4294967295 + 1` はオーバーフローしますが、`saturating_add` により `u32::MAX` に固定されます。
