struct Solution;
impl Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut cache = std::collections::HashSet::new();
        !nums.into_iter().all(|n| cache.insert(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1,2,3,1];
        let expected = true;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1,2,3,4];
        let expected = false;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![1,1,1,3,3,4,3,2,4,2];
        let expected = true;
        assert_eq!(Solution::contains_duplicate(nums), expected);
    }
}