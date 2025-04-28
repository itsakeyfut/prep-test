# Basic Syntax & Standard Library

## 1 Vec, HashMap, BinaryHeap (Priority Queue), BTreeMap

### Vec

Rust 版の動的配列

- `push`, `pop`, `insert`, `remove` などが代表的
- 配列に近いが、サイズが可変

```rs
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
println!("{:?}", v); // [1, 2, 3]
```

#### 特徴

- インデックスアクセス（`v[0]`）できる
- `.iter()` でイテレーションできる。
- `.sort()` でソートできる。

### HashMap

Rust 版の連想配列（辞書）
キーと値をペアで管理する

```rs
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("apple", 3);
map.insert("banana", 5);

if let Some(val) = map.get("apple") {
    println!("appleの個数: {}", val);
}
```

#### 特徴

- insert, get, remove が代表的
- .entry() を使うと、初期化と更新をまとめられる。これもよく使う。

### BinaryHeap (Priority Queue)

Rust 版の優先度付きキュー

- デフォルトでは最大ヒープ（大きい値から出てくる）

```rs
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);

while let Some(top) = heap.pop() {
    println!("{}", top); // 5, 3, 1 の順で取り出す
}
```

#### 特徴

- 最小ヒープにしたいから、`Reverse` を使う（これ超重要）

```rs
use std::cmp::Reverse;
heap.push(Reverse(3));
```

### BTreeMap

キーがソートされた順で格納される連想配列
(C++でいう map や、Python の SortedDict 的なもの)

```rs
let mut bmap = std::collections::BTreeMap::new();

bmap.insert(3, "three");
bmap.insert(1, "one");
bmap.insert(2, "two");

for (key, val) in &bmap {
    println!("{}: {}", key, val);
}

/*
 * 1: one
 * 2: two
 * 3: three
 */
```

## 特徴

- ある範囲だけ取り出すとか、最小・最大キーを取るときに便利

## Ownership, References (&, &mut, clone)

### Ownership

Rust では、変数には所有者がいる

- 1 つの値に対して、所有者は常に 1 つ
- 所有者がスコープを抜けると、その値は破棄される

```rs
let s = String::from("hello");
let s2 = s; // move
println!("{}", s);
println!("{}", s2);
```

```sh
error[E0382]: borrow of moved value: `s`
 --> src\main.rs:4:20
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s; // move
  |              - value moved here
4 |     println!("{}", s);
  |                    ^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s.clone(); // move
  |               ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

### References

参照ポインタを使えば、所有権を移動せずに借りることができる。

```rs
let s = String::from("hello");
let len = calculate_length(&s); // &s は参照

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

ミュータブル参照を使えば、値を変更できる借用もできる。

```rs
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### clone()

もし「値をコピーして両方使いたい」なら、`.clone()` を使う。

```rs
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy
```

## Rc, RefCell, Box (ツリー構造用)

ツリー問題（特に Binary Tree）を Rust で扱うためにはこの 3 つが超重要

### Box<>

ヒープ領域に値を置くためのスマートポインタ

ツリーのような再帰的な構造を作るときに必要。

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### Rc<T> (Reference Counted)

複数の所有者を許すスマートポインタ（例えば、親が複数の子を持つグラフ構造などで使う）

```rs
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
```

### RefCell<T>

実行時に借用ルールを緩くする箱（通常コンパイル時にチェックされる「mutable な借用」などを実行時にチェックする）

```rs
use std::cell::RefCell;

let x = RefCell::new(5);
*x.borrow_mut() += 1;
```

### 🌟【まとめ】ツリー Node を書くときの典型例

```rs
use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Node,
    right: Node,
}
```

これを型エイリアス (`type Node = ...`) しておくとツリー系が書きやすくなる！
