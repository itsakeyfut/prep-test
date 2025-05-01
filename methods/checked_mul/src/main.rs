fn main() {
    let a: u32 = 1_000_000;
    let b: u32 = 5_000;

    match a.checked_mul(b) {
        Some(result) => println!("Result: {}", result),
        None => println!("オーバーフローが発生しました")
    }
}
