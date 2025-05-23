struct Solution;
impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |prev, curr| prev ^ curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2,2,1];
        let expected = 1;
        assert_eq!(Solution::single_number(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![4,1,2,1,2];
        let expected = 4;
        assert_eq!(Solution::single_number(nums), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        let expected = 1;
        assert_eq!(Solution::single_number(nums), expected);
    }
}