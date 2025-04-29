# Deepen Knowledge: Vec, HashMap, BinaryHeap, BTreeMap

## Vec (dynamic arrya or VLA)

### Rust 設計者の視点

- C++の `std::vector` にかなり似ているが、安全性を最優先している。
- サイズと容量は分離して管理される。
- キャパシティを超えたら、自動でメモリ再確保してコピー（なので、push は平均 O(1)、最悪 O(N)）

### ポイント

- `with_capacity(n)` で初期キャパシティを指定できる（余計な再確保を防げる）
- `into_iter()` は所有権を move するイテレータになる
- `iter_mut()` は可変な参照をイテレート

### Rust 流の注意

- Vec は「ヒープにデータを置く」ので、スタック爆発を防ぐ。
- C++のような生ポインタ操作ではないので、ヒープリークやダングリングポインタが発生しない設計。

## HashMap (associative array)

### Rust 設計者の視点

- 内部実装は「ハッシュテーブル + オープンアドレッシング (Robin Hood ハッシング)」
- 「借用チェック」と「安全な並列化」を意識して設計されている。
- ハッシュアルゴリズムもデフォルトではセキュリティ重視（SipHash）になっている。
  - Python や C++だとハッシュ DoS 攻撃に弱いけど、Rust は最初から守っている。

### ポイント

- `.entry(key).or_insert(val)` で、存在しないときだけ初期化できる (LeetCode 頻出テク)
- ハッシュ関数を変えたいなら、`HashMap<K, V, S>` で第三型引数を指定可能

## BinaryHeap (Priority Queue)

### Rust 設計者の視点

- 内部は「配列ベースのヒープ」
- BinaryHeap は常に最大ヒープになっている（最小ヒープにするには工夫が必要）。

### ポイント

- 最小ヒープにするには、要素を `Reverse(val)` でラップする
- `push` と `pop` は O(logN)
- `peek()` でトップを見れる (pop はしない)

### 注意

- BinaryHeap は PartialOrd しか要求しないので、独自型でも簡単に扱える。

## BTreeMap (Sorted associate array)

### Rust 設計者の視点

- 「AVL 木」や「Red-Black 木」ではなく、BTree ベース
- これは「キャッシュ効率」を優先した選択。（CPU のキャッシュミスを減らす設計）

### ポイント

- `range(a..b)` で範囲走査ができる
- `first_key_value()`, `last_key_value()` で最小・最大を取得できる

### Rust 流の注意

- 大量データ（1000 万件以上）なら、`HashMap` より `BTreeMap` の方がトータル性能が良い場合もある（CPU キャッシュ効率が効いてくる）

---

# Deepen Knowledge: Ownership, References, Rc, RefCell, Box

## Ownership, References

### Rust 設計者の視点

- 「コンパイラによるメモリ安全性保証」が最大の目標
- "Move Semantics"が言語の中心
- C++の "Rule of Five" みたいな複雑なコピー管理を Rust では禁止して、すべてムーブ中心にした。

### ポイント

- 所有権は絶対１つ
- 参照借用（&）は何個でも OK。ただし可変参照 (&mut)は一個しか作れない。
  - 変数が三つあって、全て参照借用なら OK
  - 変数が三つあって、１つでも可変参照があれば、残りは参照借用できない。

## Rc, RefCell, Box

| 種類       | 説明                   | 用途                     |
| ---------- | ---------------------- | ------------------------ |
| Box<T>     | 単なるスマートポインタ | ヒープ確保、再帰型に必要 |
| Rc<T>      | 参照カウントポインタ   | 複数オーナー             |
| RefCell<T> | 実行時可変借用         | ツリー構造の可変アクセス |

### 設計思想

- Box
  - 所有者は１つ、可変にするなら `Box<RefCell<T>>`
- Rc
  - 複数オーナー許可（Clone でカウント++）
- RefCell
  - 「実行時チェック」でミュータブルを緩める
