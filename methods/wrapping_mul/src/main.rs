fn main() {
    let a: u32 = 1_000_000;
    let b: u32 = 5_000;

    let result = a.wrapping_mul(b);

    println!("{}", result); // 705032704
}