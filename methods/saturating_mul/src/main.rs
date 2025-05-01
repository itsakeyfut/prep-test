fn main() {
    let a: u32 = 1_000_000;
    let b: u32 = 5_000;

    let result = a.saturating_mul(b);

    println!("{}", result); // 4294967295
}
