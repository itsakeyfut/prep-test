fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn sum(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + sum(n -1)
    }
}

fn main() {
    println!("{}", factorial(5)); // 120
    println!("{}", sum(5));       // 15
}
