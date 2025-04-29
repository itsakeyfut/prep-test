fn factorial_tail(n: u32, acc: u32) -> u32 {
    if n == 0 { acc }
    else { factorial_tail(n - 1, acc * n) }
}

fn main() {
    println!("{}", factorial_tail(5, 1)); // 120
}
