# Ownership

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
