fn main() {
    let a: u32 = u32::MAX;
    let b: u32 = 1;

    let result = a.saturating_add(b);

    println!("{}", result); // 4294967295
}
