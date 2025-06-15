struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        nums.iter()
            .for_each(|&num| *map.entry(num).or_insert(0) += 1);
        map.iter().fold(0, |acc, (&num, &cnt)| {
            map.get(&(num + 1)).map_or(acc, |c| acc.max(cnt + c))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let expected = 5;

        assert_eq!(Solution::find_lhs(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 2, 3, 4];
        let expected = 2;

        assert_eq!(Solution::find_lhs(nums), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 1, 1, 1];
        let expected = 0;

        assert_eq!(Solution::find_lhs(nums), expected);
    }
}
