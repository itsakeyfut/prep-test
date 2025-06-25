struct Solution;
impl Solution {
    fn third_max(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut unique_nums: Vec<i32> = set.into_iter().collect();
        unique_nums.sort_unstable_by(|a, b| b.cmp(a));

        if unique_nums.len() >= 3 {
            unique_nums[2]
        } else {
            unique_nums[0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3,2,1];
        let expected = 1;
        assert_eq!(Solution::third_max(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1,2];
        let expected = 2;
        assert_eq!(Solution::third_max(nums), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![2,2,3,1];
        let expected = 1;
        assert_eq!(Solution::third_max(nums), expected);
    }
}