# Box<T>

ヒープ領域に値を置くためのスマートポインタ。

ツリーのような再帰的な構造を作るときに必要。

```rs
enum List {
    Cons(i32, Box<List>),
    Nil
}
```
