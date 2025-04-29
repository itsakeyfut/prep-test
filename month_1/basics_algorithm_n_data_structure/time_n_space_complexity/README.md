# Time and Space Complexity（時間・空間計算量）

## 🧠 なぜ重要？

アルゴリズムの効率を測る物差し

入力が増えたときの「処理時間」や「メモリ使用量」の増え方を分析する。

Rust では LeetCode や競技プログラミングをする際にも最重要指標のひとつ。

## ⏱️ 時間計算量（Time Complexity）

### 代表的なオーダー（Big-O 記法）

| 計算量        | 例                           | 意味                     |
| :------------ | :--------------------------- | :----------------------- |
| $O(1)$        | `let x = v[0];`              | 一定時間で終わる処理     |
| $O(n)$        | `for x in v {}`              | データ全体を 1 回走査    |
| $O(\log n)$   | 二分探索                     | データを半分に絞りながら |
| $O(n \log n)$ | 高速ソート（MergeSort など） | 全体走査＋半分分割       |
| $O(n^2)$      | 二重ループ                   | 全組み合わせチェック     |
| $O(2^n)$      | 再帰的な全探索               | サブセット生成など       |

### 実例

```rs
// O(n)
fn linear_sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

// O(n^2)
fn find_pairs(v: &[i32]) {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            println!("({}, {})", v[i], v[j]);
        }
    }
}
```

## 🧮 空間計算量（Space Complexity）

- 処理中に使う追加のメモリの差
- 入力に比例するか？（線形空間）か、それとも固定量だけか？（定数空間）

### 例

```rs
fn sum(v: &[i32]) -> i32 {
    let mut total = 0; // O(1)
    for &x in v {
        total += x;
    }
    total
}
```

```rs
fn cumulative_sum(v: &[i32]) -> Vec<i32> {
    let mut result = Vec::new(); // O(n)
    let mut total = 0;
    for &x in v {
        total += x;
        result.push(total);
    }
    result
}
```

## ⚖️ トレードオフの感覚

- 高速化したい
  - 空間を余計に使ってもいい
- メモリが厳しい
  - 時間がかかっても OK

たとえば：

- ハッシュマップ：探索高速 O(1) だが空間消費量多い
- バイナリサーチ：空間は少なくても済むがソートが必要

## 📌 計算量を測る Rust の実験（例）

```rs
fn main() {
    let v: Vec<u64> = (0..10_000_000).collect();
    let now = std::time::Instant::now();
    let sum: u64 = v.iter().sum();
    println!("sum = {}, time = {:?}", sum, now.elapsed()); // sum = 49999995000000, time = 31.2907ms
}
```

## ✅ ここまでで理解すべきこと

- アルゴリズムは速さと軽さのバランスが命
- Rust でコーディングする際、計算量を常に意識する
- O(1)、O(n)、O(log n)、O(n²)、O(2ⁿ) などの「感覚」をつけることが大切！
