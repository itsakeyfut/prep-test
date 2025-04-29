# fold()

## 【累積和問題 001】

整数の配列 nums が与えられます。
nums の累積和を計算して、累積和配列を返してください。
例：

```
nums = [1, 2, 3, 4]
=> 累積和 = [1, 3, 6, 10]
```

```rs
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let cumsum = v.iter().fold(
        Vec::new(),
        |mut acc, x| {
            let last = *acc.last().unwrap_or(&0);
            acc.push(last + x);
            acc
        }
    );
    println!("{:?}", cumsum); // [1, 3, 6, 10, 15]
}
```
