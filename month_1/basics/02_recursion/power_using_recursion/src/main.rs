fn power(x: i64, n: u32) -> i64 {
    // Base case
    if n == 0 {
        return 1;
    }
    // Recursion case
    x * power(x, n - 1)
}

fn main() {
    println!("2^5 = {}", power(2, 5)); // 32
}