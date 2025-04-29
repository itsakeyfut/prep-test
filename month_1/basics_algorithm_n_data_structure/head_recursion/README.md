# Head Recursion（戦闘再帰）

## ✅ Head Recursion とは？

再帰呼び出しが関数の最初に来ている形の再帰です。

つまり、再帰の戻り値を使って何かをする再帰です。

## 📘 例：1 から n までの出力を行う Head Recursion

```rs
fn print_head(n: u32) {
    if n > 0 {
        print_head(n - 1);
        println!("{}", n);
    }
}

/*
* 1
* 2
* 3
* 4
* 5
*/
```

## 🤔 解説

- 再帰呼び出しから戻ってきてから処理を行う
- よって、処理順は呼び出しとは逆（スタック構造で LIFO）

## ✅ 対比：Tail Recursion（末尾再帰）

```rs
fn print_head(n: u32) {
    if n > 0 {
        println!("{}", n);
        print_head(n - 1);
    }
}

/*
* 5
* 4
* 3
* 2
* 1
*/
```

## 🚀 応用でよく使われる場面

- ツリー構造の後順・中順探索（例：InOrder Traversal）
- 再帰的なデータ構造の後処理を重視する場面（例：戻り値を使って合成する）

## 🧪 演習

以下の関数 `sum` を Head Recursion にしてください。

```rs
fn sum_tail(n: u32, acc: u32) -> u32 {
    if n == 0 { acc }
    else { sum_tail(n - 1, acc + n) }
}
```

ヒント：

- Head Recursion では n + sum(n - 1) のように、戻り値を使って処理します。
