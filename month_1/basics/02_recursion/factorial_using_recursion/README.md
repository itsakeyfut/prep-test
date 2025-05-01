# Factorial using Recursion

## Explanation

The factorial of a non-negative integer n is the product of all positive integers less than or equal to n. It's denoted by n! and defined as:

- n! = n × (n-1) × (n-2) × ... × 2 × 1
- Special case: 0! = 1 (by definition)

Recursion is a perfect fit for factorial calculation because the problem can be broken down into smaller subproblems:

- n! = n × (n-1)!

In Rust, here's how we implement factorial using recursion:

```rs
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
```

## Keypoints

1. Base Case: Every recursive function needs a base case to prevent infinite recursion. Here, when n = 0, we return 1.
2. Recursive Case: For n > 0, we multiply n by the factorial of (n-1).
3. Call Stack: Each recursive call adds a frame to the call stack, which can lead to stack overflow for large values of n.
4. Rust specifics: We use u64 to handle larger factorials, but even this will overflow for n > 20.

## Practice Exercise

1. Implement the factorial function in Rust.
2. Add checks to handle potential overflow.
3. Compare the recursive implementation with an iterative implementation.

## Solution

### 1. Implementing the Factorial Function in Rust

Here's basic recursive implementations:

```rs
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 5;
    println!("Factorial of {} is: {}", n, factorial(n));
}
```

### 2. Adding Checks to Handle Potential Overflow

Here's an implementation using `checked_mul` to handle overflow:

```rs
fn factorial_safe(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    } else {
        factorial_safe(n - 1).and_then(|prev| n.checked_mul(prev))
    }
}

fn main() {
    let n = 20;
    match factorial_safe(n) {
        Some(result) => println!("Factorial of {} is: {}", n, result), // Factorial of 20 is: 2432902008176640000
        None => println!("Factorial of {} causes overflow!", n)
    }

    // This will overflow
    let large_n = 21;
    match factorial_safe(large_n) {
        Some(result) => println!("Factorial of {} is: {}", large_n, result),
        None => println!("Factorial of {} causes overflow!", large_n) // Factorial of 21 causes overflow!
    }
}
```

### 3. Comparing Recursive vs Iterative Implementation

Here are both implementations are side by side:

```rs
// Recursive factorial with overflow hadnling: O(n)
fn factorial_recursive(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    } else {
        factorial_recursive(n - 1).and_then(|prev| n.checked_mul(prev))
    }
}

// Iterative factorial with overflow handling: O(1)
fn factorial_iterative(n: u64) -> Option<u64> {
    let mut result = 1u64;

    for i in 1..=n {
        result = match result.checked_mul(i) {
            Some(val) => val,
            None => return None
        };
    }

    Some(result)
}


fn main() {
    let n = 15;

    // Compare results
    println!("Recursive factorial of {}: {:?}", n, factorial_recursive(n)); // Recursive factorial of 15: Some(1307674368000)
    println!("Iterative factorial of {}: {:?}", n, factorial_iterative(n)); // Iterative factorial of 15: Some(1307674368000)

    // Compare execution for a larger value
    let large_n = 20;
    println!("Recursive factorial of {}: {:?}", large_n, factorial_recursive(large_n)); // Recursive factorial of 20: Some(2432902008176640000)
    println!("Iterative factorial of {}: {:?}", large_n, factorial_iterative(large_n)); // Iterative factorial of 20: Some(2432902008176640000)
}
```

## Comparison Analysis

1. Memory Usage:

- The recursive solution uses O(n) space on the call stack (one stack frame per recursive call)
- The iterative solution uses O(1) space (just the result variable)

2. Performance:

- The recursive solution may be slightly slower due to function call overhead
- For very large values, the recursive solution risks stack overflow

3. Readability

- The recursive solution directly maps to the mathematical definition
- The iterative solution is more imperative but avoids stack issues

4. Rust-specific advantages:

- The iterative solution is more idiomatic in Rust for this particular problem
- Using `checked_mul` in both approaches prevents runtime panics from overflow

For most practical purposes, the iterative solution is preferred in Rust because:

- It's more efficient with memory
- It avoids potential stack overflow
- Rust's ownership system works more naturally with iterative solutions
