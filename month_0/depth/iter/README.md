# Iterator の本質

Rust では「イテレータ」はただの値の生産者です。

```rs
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- `next()` を呼ぶと `Option<Item>` が返るだけ。
- 内部的には、`&mut self` に状態を持っていて、自分で動いていく。
- for ループは、`Iterator` を自動で `next()` し続けるだけ。

▶️ だから、Rust 流では

- for item in iterator { ... }
- iterator.map(...),filter(...).collect::<Vec<\_>>()

みたいに、イテレータをパイプラインのようにつなげるのか正義

🔵 Rust 流イテレータの美学まとめ

| 美学                                         | 理由                                               |
| -------------------------------------------- | -------------------------------------------------- |
| Iterator を組み合わせる                      | 処理を小さく分け、Composable（合成可能）にするため |
| クロージャで関数型プログラミング             | map/filter/fold でラムダ式を使い捨てる             |
| collect()で最終形にする                      | Vec, HashSet, HashMap に変換可能                   |
| 無駄なメモリコピーをしない                   | .iter(), .into_iter()を使い分ける                  |
| 自作 Iterator で独自データ構造も自然に使える | 一貫したインターフェースになる                     |

# イテレータの基本

## イテレータの基本形

Rust ではイテレータは「`next()`を呼び続けるだけ」の仕組み。
実は、`for item in iter {} `はコンパイラが展開しているだけ。

```rs
let mut iter = vec![1, 2, 3].into_iter(); // Iteratorに変換

while let Some(x) = iter.next() {
    println!("{X}");
}
```

✅ `next()` は `Option<T>` を返すから、Some でループ、None で終了するだけ！

## for/while と next()の違い

| パターン                        | 内容                                 |
| ------------------------------- | ------------------------------------ |
| for x in iter                   | イテレータを消費する（所有権を取る） |
| while let Some(x) = iter.next() | 手動で次の要素を取る。柔軟性が高い   |
| .iter()                         | 参照(&) のイテレータ                 |
| .into_iter()                    | 所有権移動のイテレータ               |

```rs
let v = vec![1, 2, 3];

for x in &v { /* 借用 */ }
for x in v { /* 所有権を奪う */ }
```

▶️ どちらかを選ぶのは「借用したいか」「消費したいか」で決める！

## map / filter / fold / collect

Rust のイテレータは関数型スタイルでパイプラインを作れる。

```rs
let v = vec![1, 2, 3, 4, 5];

// map: 各要素を変換する
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

// filter: 条件を満たすものだけ通す
let even: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
assert_eq!(even, vec![2, 4]);
```

✅ `collect()` はイテレータの最終形をまとめる関数
✅ `map` も `filter` も、イテレータを作るだけで、実際にはまだ計算されない！（遅延評価！）

## イテレータアダプタをつなぎまくる

Rust 流では、イテレータをいかにキレイにつなぐかが勝負。

```rs
let v = vec![1, 2, 3, 4, 5];

let result: Vec<_> = v.into_iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .take(2)
    .collect();

assert_eq!(result, vec![1, 9]);
```

- 奇数だけフィルタ
- それを 2 乗
- 最初の 2 個だけ取る
- ベクタにまとめる

▶️ これが Rust らしい書き方！

## 自作 Iterator を作って理解を深める

最後に、自分でイテレータを作ってみよう！

```rs
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let result = Some(self.curr);
        self.curr = self.next;
        self.next = new_next;
        result
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for x in fib.take(10) {
        println!("{x}");
    }
}
```

✅ Iterator トレイトを実装すると、for で使える！
✅ 自作データ構造でも標準イテレータっぽく扱える！
