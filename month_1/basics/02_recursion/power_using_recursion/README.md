# Power using Recursion

## 説明

累乗（指数計算）は再帰に適したもう一つの古典的な問題です。
タスクは x^n を計算することです。

再帰的アプローチは以下の数学的性質に基づいています：

- x^n = x × x^(n-1)
- ベースケース: x^0 = 1（どんな数も 0 乗は 1）

Rust での実装は以下の通りです：

```rs
fn power(x: i64, n: u32) -> i64 {
    // Base case
    if n == 0 {
        return 1;
    }
    // Recursion case
    x * power(x, n - 1)
}

fn main() {
    println!("2^5 = {}", power(2, 5)); // 32
}
```

## 最適化：高速累乗アルゴリズム

分割統治法を使って最適化できます：

- n が偶数の場合: x^n = (x^(n/2))²
- n が奇数の場合: x^n = x × (x^(n/2))²

これにより再帰呼び出しの回数が O(n)から O(log n)に減少します：

```rs
fn fast_power(x: i64, n: u32) -> i64 {
    if n == 0 {
        return 1;
    }

    let half = fast_power(x, n / 2);

    if n % 2 == 0 {
        // nが偶数
        half * half
    } else {
        // nが奇数
        x * half * half
    }
}
```

## 練習問題

1. 両方の累乗関数を Rust で実装する
2. `checked_mul` を使ってオーバーフロー保護を追加する
3. 大きな指数に対して両方のアプローチのパフォーマンスを比較する
4. 高速累乗アルゴリズムの反復版を実装する

## 解答

1. 両方の累乗関数を Rust で実装

```rs
fn power(x: i64, n: u32) -> i64 {
    if n == 0 {
        return 1;
    }
    x * power(x, n - 1)
}

fn fast_power(x: i64, n: u32) -> i64 {
    if n == 0 {
        return 1;
    }

    let half = fast_power(x, n / 2);

    if n % 2 == 0 {
        half * half
    } else {
        x * half * half
    }
}

fn main() {
    let base = 2;
    let exp = 10;
2^10 = 1024
    println!("{}^{} = {}", base, exp, power(base, exp)); // 2^10 = 1024
    println!("{}^{} = {}", base, exp, fast_power(base, exp)); // 2^10 = 1024
}
```

2. `checked_mul` を使ったオーバーフロー保護の追加

```rs
fn power_safe(x: i64, n: u32) -> Option<i64> {
    if n == 0 {
        return Some(1);
    }

    power_safe(x, n - 1).and_then(|prev| x.checked_mul(prev))
}

fn fast_power_safe(x: i64, n: u32) -> Option<i64> {
    if n == 0 {
        return Some(1);
    }

    let half = fast_power_safe(x, n / 2);
    let half_squared = half?.checked_mul(half?)?;

    if n % 2 == 0 {
        Some(half_squared)
    } else {
        x.checked_mul(half_squared)
    }
}

fn main() {
    let base = 3;
    let exp = 20;

    match power_safe(base, exp) {
        Some(result) => println!("{}^{} = {}", base, exp, result), // 3^20 = 3486784401
        None => println!("{}^{} はオーバーフローします！", base, exp)
    }

    match fast_power_safe(base, exp) {
        Some(result) => println!("{}^{} = {}", base, exp, result), // 3^20 = 3486784401
        None => println!("{}^{} はオーバーフローします！", base, exp)
    }
}
```

3. 両方のアプローチのパフォーマンス比較

```rs
use std::time::{Duration, Instant};

fn measure_time<F>(f: F) -> Duration
where
    F: FnOnce() -> i64
{
    let start = Instant::now();
    let _ = f();
    start.elapsed()
}

fn main() {
    let base = 2;
    let exp = 25;

    let regular_time = measure_time(|| power(base, exp));
    println!("通常の累乗関数の時間：{:?}", regular_time);

    let fast_time = measure_time(|| fast_power(base, exp))
    println!("高速累乗の時間：{:?}", fast_time);

    println!("速度向上：{:.2}倍",
        regulat_time.as_nanos() as f64 / fast_time.as_nanos() as f64);
}
```

より大きな指数に対しては、パフォーマンスの違いが劇的に現れます。
単純な再帰的アプローチは O(n)の計算量を持ちますが、高速累乗アルゴリズムは O(log n)の計算量です。

4. 高速累乗アルゴリズムの反復版

```rs
fn iterative_fast_power(x: i64, mut n: u32) -> i64 {
    let mut result = 1;
    let mut current = x;

    while n > 0 {
        // n が奇数なら、結果に x^(2^k) の現在の値を掛ける
        if n % 2 == 1 {
            result *= current;
        }

        // 次の繰り返しのために現在の値を2乗する
        current *= current;

        // 2 で整数除算
        n /= 2;
    }

    result
}

fn iterative_fast_power_safe(x: i64, mut n: u32) -> Option<i64> {
    let mut result = 1;
    let mut current = x;

    while n > 0 {
        if n % 2 == 1 {
            result = result.checked_mul(current);
        }

        current = current.checked_mul(current)?;
        n /= 2;
    }

    Some(result)
}

fn main() {
    let base = 2;
    let exp = 10;

    println!("{}^{} = {}",
        base, exp, iterative_fast_power(base, exp));

    match iterative_fast_power_safe(base, exp) {
        Some(result) => println!("{}^{} = {}", base, exp, result),
        None => println!("{}^{} はオーバーフローします！", base, exp)
    }
}
```

反復版にはいくつかの利点があります：

1. 再帰がないため、スタックオーバーフローのリスクがない
2. パフォーマンスが向上する（関数呼び出しのオーバーヘッドがない）
3. より大きな指数を扱える（データ型によってのみ制限される）

実際には、これらの利点から、高速累乗アルゴリズムの反復版は本番コードで好まれることが多いです。
