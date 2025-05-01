fn main() {
    let a: u32 = 0;
    let b: u32 = 1;

    let result = a.saturating_sub(b);

    println!("{}", result); // 0
}