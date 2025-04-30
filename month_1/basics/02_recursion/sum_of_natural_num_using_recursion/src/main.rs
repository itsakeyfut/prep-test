fn sum(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + sum(n - 1)
    }
}

fn main() {
    let result = sum(5);
    println!("{}", result); // 15
}
