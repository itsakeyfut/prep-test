fn main() {
    let mc91 = |mut f: &dyn Fn(i32) -> i32, n: i32| -> i32 {
        if n > 100 {
            n - 10
        } else {
            f(f(n + 11))
        }
    };

    // 再帰的クロージャには trick が必要（Yコンビネータ or 関数ポインタ）
    fn wrapper(n: i32) -> i32 {
        fn inner(n: i32) -> i32 {
            if n > 100 {
                n - 10
            } else {
                inner(inner(n + 11))
            }
        }
        inner(n)
    }

    for i in 85..=100 {
        println!("wrapper({}) = {}", i, wrapper(i));
    }
}