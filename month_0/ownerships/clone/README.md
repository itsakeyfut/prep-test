# clone

もし「値をコピーして両方使いたい」なら、`.clone()` を使う。

```rs
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy
```
