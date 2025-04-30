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
