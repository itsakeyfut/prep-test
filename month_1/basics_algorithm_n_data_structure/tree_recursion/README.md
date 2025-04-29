# Tree Recursion (木構造再帰)

## ✅ Tree Recursion とは？

1 つの関数の中で複数回再帰を呼び出す再帰のことです。

つまり、同じ関数を複数回呼ぶことで「木」構造のような再帰呼び出し関係が生まれます。

## 📘 例：フィボナッチ数列（再帰的定義）

````rs
fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

再帰の構造は以下のようになります（例：fib(3)）：

```rs
         fib(3)
        /      \
    fib(2)    fib(1)
   /     \
fib(1)  fib(0)
````

=> 木構造になっている

## 🤔 解説

- Tree Recursion はとても強力ですが、計算量が指数的に増加するため注意！
- 重複計算を避けたい場合は、**メモ化（メモリ最適化）** が有効

## 🧠 設計視点

- Tree Recursion は、分岐的な選択肢をすべて探索するような問題（バックトラッキングなど）で使われます。
- DFS（深さ優先探索）、パーミュテーション生成などが好例です

## 🔁 Rust で Tree Recursion を使ったパターン

### 🎲 例：再帰的に文字列を 2 回出力する

```rs
fn tree_rec(n: u32) {
    if n > 0 {
        println!("{}", n);
        tree_rec(n - 1);
        tree_rec(n - 1);
    }
}
```

この関数は以下のように 2^n - 1 回出力します。

```
tree_rec(3) の出力：
3
2
1
1
2
1
1
```

## 🧪 演習：Tree Recursion による「すべての部分集合」の列挙

```rs
fn subsets(s: &str, curr: String) {
    if s.is_empty() {
        println!("{}", curr);
    } else {
        let (first, rest) = s.split_at(1);
        subsets(rest, curr.clone()); // 含まないパターン
        subsets(rest, curr + first); // 含むパターン
    }
}

fn main() {
    subsets("abc", "".to_string());
}

/*
 * c
 * b
 * bc
 * a
 * ac
 * ab
 * abc
 */
```
