# 🎯 目標：「高度な Rust イテレータ」を理解する

出てくる技：

| 技                                                      | 内容                            |
| ------------------------------------------------------- | ------------------------------- |
| イテレータ合成                                          | 複数のイテレータを組み合わせる  |
| 自作アダプタ（map, filter の自作版）                    | Iterator trait をフル活用する   |
| by_ref() を使ったイテレータ分割                         | 一つのイテレータを 2 回使う裏技 |
| Lifetime 付きイテレータ                                 | 借用して走らせるイテレータ      |
| GAT (Generic Associated Types) を使った次世代イテレータ | 超上級                          |

## イテレータ合成

Rust のイテレータは無限に合成できるのが強み！

```rs
fn composite_iter() {
    let v = vec![1, 2, 3, 4, 5];

    let sum = v.iter()
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0) // 6 だけフィルタリング
        .sum::<i32>();

    println!("{}", sum); // 6
}
```

これは内部的には一度も Vec を作っていない

- map
  - 2 倍
- filter
  - 3 の倍数だけ
- sum
  全部足して終わり

中間で無駄なメモリ確保をしない。
すべては「遅延評価（lazy evaluation）」で動いている。
Rust はイテレータにこの「パイプライン型」を徹底してる。

## 自作イテレータアダプタ

例：「偶数だけ返すイテレータアダプタ」

```rs
struct Even<I> {
    iter: I,
}

impl<I> Iterator for Even<I>
where
    I: Iterator<Item = i32>
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if x % 2 == 0 {
                return Some(x);
            }
        }
        None
    }
}

fn custom_iter_adapter() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let even = Even { iter: v.into_iter() };

    for num in even {
        println!("{}", num);
        /*
         * 2
         * 4
         * 6
         */
    }
}
```

## `by_ref()` を使ってイテレータを二回使う裏技

```rs
fn use_twice_iter() {
    let mut v = vec![1, 2, 3, 4, 5];

    let mut iter = v.iter();

    let first_two: Vec<_> = iter.by_ref().take(2).collect();
    let rest: Vec<_> = iter.collect();

    println!("{:?}", first_two); // [1, 2]
    println!("{:?}", rest);      // [3, 4, 5]
}
```

`iter` を一回で 2 回に分割できる。

内部は「借用」だけしてて、同じイテレータを動かしている。

- `by_ref()`
  - `&mut self` を返す
- だから元のイテレータも動く

## ライフタイム付きイテレータ

参照を返すイテレータもできる。

```rs
struct Lines<'a> {
    text: &'a str,
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.text.find('\n') {
            let line = &self.text[..pos];
            self.text = &self.text[pos + 1..];
            Some(line)
        } else if !self.text.is_empty() {
            let line = self.text;
            self.text = "";
            Some(line)
        } else {
            None
        }
    }
}

fn iter_with_lifetime() {
    let text = "hello\nworld\nRust";

    let mut lines = Lines { text };

    while let Some(line) = lines.next() {
        println!("{}", line);
    }

    /*
     * hello
     * world
     * Rust
     */
}
```

## GAT イテレータ

Rust の「GAT（Generic Associated Types）」を使うと、より柔軟なイテレータが作れる。

「返すアイテムの型が、借用ごとに代わる」
