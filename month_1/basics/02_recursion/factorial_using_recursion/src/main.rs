fn factorial(n: u64) -> u64 {
    // Base case
    if n == 0 {
        return 1;
    }
    // Recursive case
    n * factorial(n - 1)
}

fn main() {
    println!("Factorial of 5 is: {}", factorial(5)); // 120
}
