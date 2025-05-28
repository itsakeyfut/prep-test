struct NumArray {
    monotonic_stack: Vec<i32>
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            monotonic_stack: [0].iter().chain(nums.iter())
                .scan(0, |state, val| {
                    *state += val;
                    Some(*state)
                }).collect::<Vec<i32>>(),
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.monotonic_stack[(right + 1) as usize] - self.monotonic_stack[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}