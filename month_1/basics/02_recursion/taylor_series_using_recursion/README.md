# Taylor Series using Recursion

## 説明

テイラー級数は、ある点における関数の導関数から導かれる項の無限和として関数を表現する方法です。
例えば、x=0 周りの e^x（マクローリン級数とも呼ばれる）のテイラー級数は：
e^x = 1 + x + x²/2! + x³/3! + x⁴/4! + ...
点 a における関数 f(x)のテイラー級数展開の一般式は：
f(x) = f(a) + f'(a)(x-a)/1! + f''(a)(x-a)²/2! + f'''(a)(x-a)³/3! + ...

## Rust での実装

e^x のテイラー級数を計算する再帰関数を実装してみましょう：

```rs
fn e_power_x_term(x: f64, n: u32) -> f64 {
    // n番目の項を計算: x^n / n!
    if n == 0 {
        return 1.0;
    }

    // x^n / n!を計算
    x.powi(n as i32) / factorial(n) as f64
}

fn e_power_x_series(x: f64, n: u32) -> f64 {
    // ベースケース
    if n == 0 {
        return 1.0;
    }

    // 再帰ケース: n番目の項まで合計
    e_power_x_series(x, n-1) + e_power_x_term(x, n)
}

fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    n * factorial(n-1)
}
```

## 最適化アプローチ

上記のアプローチでは、各項を独立して再計算しています。前の項に基づいて各項を計算することで、より効率的にできます：

```rs
fn e_power_x_optimized(x: f64, n: u32) -> f64 {
    fn taylor_term(x: f64, n: u32, previous_term: f64) -> f64 {
        if n == 0 {
            return previous_term;
        }

        // 次の項を計算: previous_term * x / n
        let current_term = previous_term * x / n as f64;

        // この項を前のすべての項の合計に追加
        current_term + taylor_term(x, n-1, current_term)
    }

    // 最初の項 = 1.0から開始
    1.0 + taylor_term(x, n, 1.0)
}
```

# ホーナー法を使ったテイラー級数 - 再帰

## 説明

ホーナー法は多項式を評価するための効率的なアルゴリズムです。各項を別々に計算する代わりに、共通の乗算を括り出します。
テイラー級数の場合：
e^x = 1 + x + x²/2! + x³/3! + ...
ホーナー法はこれを次のように変換します：
e^x = 1 + x(1 + x/2(1 + x/3(1 + x/4(...))))

## Rust での実装

```rs
fn e_power_x_horner(x: f64, n: u32) -> f64 {
    // ベースケース
    if n == 0 {
        return 1.0;
    }

    // ホーナー法を再帰的に適用
    1.0 + x / n as f64 * e_power_x_horner(x, n-1)
}
```

このアプローチは計算的にも数値的にもより効率的です。

## テイラー級数の反復法

反復的手法を使ってテイラー級数計算を実装することもできます：

```rs
fn e_power_x_iterative(x: f64, n: u32) -> f64 {
    let mut sum = 1.0;  // 最初の項
    let mut term = 1.0;

    for i in 1..=n {
        term *= x / i as f64;  // 前の項から次の項を計算
        sum += term;
    }

    sum
}

fn e_power_x_horner_iterative(x: f64, n: u32) -> f64 {
    let mut result = 1.0;

    for i in (1..=n).rev() {
        result = 1.0 + x / i as f64 * result;
    }

    result
}
```

実際には、効率性とスタックオーバーフローのリスク回避のため、反復版がよく好まれます。
