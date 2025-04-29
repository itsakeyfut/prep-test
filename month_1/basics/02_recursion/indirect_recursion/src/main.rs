fn is_even(n: u32) -> bool {
    if n == 0 {
        true
    } else {
        is_odd(n - 1)
    }
}

fn is_odd(n: u32) -> bool {
    if n == 0 {
        true
    } else {
        is_even(n - 1)
    }
}

fn main() {
    println!("{}", is_even(10)); // true
    println!("{}", is_even(9));  // true
}

