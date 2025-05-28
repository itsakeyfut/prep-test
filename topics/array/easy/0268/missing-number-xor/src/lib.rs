struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut missing = n as i32;

        for (i, num) in nums.into_iter().enumerate() {
            missing ^= i as i32 ^ num;
        }

        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3,0,1];
        let expected = 2;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![0,1];
        let expected = 2;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![9,6,4,2,3,5,7,0,1];
        let expected = 8;
        assert_eq!(Solution::missing_number(nums), expected);
    }
}