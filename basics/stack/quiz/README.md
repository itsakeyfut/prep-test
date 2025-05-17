# Stack

## 配列の作成

```rs
let lookup: HashMap<char, char> = [
    ('{', '}'),
    ('[', ']'),
    ('(', ')')
]
.iter()
.cloned()
.collect();
```

### `.iter()`

このメソッドは、配列の参照のイテレータを生成します。つまり、型は `std::slice::Iter<'_, (char, char)>` となり、要素の型は `&(char, char)` になります

### `.cloned()`

`.iter()` によって得られるのは参照 `&(char, char)` なので、そのまま `collect()` しても `HashMap<&char, &char>` になります。

しかし、欲しいのは `HashMap<char, char>`。そこで `.cloned()` を使って `&(char, char)` を `(char, char)` にコピー（クローン）します。これは `Copy` トレイトが実装されている `char` 型だから可能です。

### `.collect()`

最終的に、イテレータから `HashMap` を生成します。型注釈 let lookup: `HashMap<char, char>` があることで、Rust は何に `collect()` するかを推論できます。

この処理が必要な理由は、型の整合性と所有権モデルに従うために必要なステップだからです。

## 配列全体をムーブする

```rs
let lookup: HashMap<char, char> = [
    ('{', '}'),
    ('[', ']'),
    ('(', ')')
]
.into_iter()
.collect();
```

## 開き括弧・閉じ括弧を見つける

```rs
for char in chars.chars() {
    if lookup.contains_key(&char) {
        stack.push(lookup[&char]);
    } else if lookup.values().any(|&v| v == char) {
        if stack.is_empty() || stack.pop() != Some(char) {
            return false;
        }
    }
}
```

### `for char in chars.chars()`

- 入力文字列 `chars` を 1 文字ずつ処理するループです。
- `chars.chars()` は `&str` を `char` のイテレータに変換します。

### `if lookup.contains_key(&char)`

- この `char` が 開き括弧（`{`, `[`, `(` のいずれか）かどうかを確認。
- 開き括弧であれば：

```rs
stack.push(lookup[&char]);
```

- スタックに、対応する閉じ括弧（例：`}`）を push します。
- こうすることで「次にこの閉じ括弧が来るはずだよ」という情報を保存しておけます。

### `else if lookup.values().any(|&v| v == char)`

- `char` が 閉じ括弧（`}`, `]`, `)` のいずれか）であるか確認。

```rs
if stack.is_empty() || stack.pop() != Some(char) {
    return false;
}
```

- スタックが空 → 対応する開き括弧がなかった → 不正な閉じ括弧
- スタックから pop した括弧と現在の `char` が一致しない → ペアが合っていない
- このどちらかならフォーマットが壊れているので `false` を返します。

### ✅ 最後に `stack.is_empty()`

ループ終了後にスタックが空であれば、すべての括弧が正しく閉じられていたことになります。
