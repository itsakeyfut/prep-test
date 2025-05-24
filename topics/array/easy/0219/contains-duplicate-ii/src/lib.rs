struct Solution;
impl Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k <= 0 {
            return false;
        }

        let mut seen = std::collections::HashMap::with_capacity(nums.len());

        for (i, n) in nums.into_iter().enumerate() {
            let j = seen.entry(n).or_insert(i);
            if *j != i && i - *j <= k as usize {
                return true;
            } else {
                *j = i;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1,2,3,1];
        let k = 3;
        let expected = true;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1,0,1,1];
        let k = 1;
        let expected = true;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![1,2,3,1,2,3];
        let k = 2;
        let expected = false;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), expected);
    }
}