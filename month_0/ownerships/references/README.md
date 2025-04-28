# References

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
