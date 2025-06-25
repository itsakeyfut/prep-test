struct Solution;
impl Solution {
    fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score_with_idx: Vec<(i32, usize)> = score.iter().cloned().zip(0..).collect();

        score_with_idx.sort_by(|a, b| b.0.cmp(&a.0));

        let mut result = vec![String::new(); score.len()];
        for (rank, &(_, idx)) in score_with_idx.iter().enumerate() {
            result[idx] = match rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (rank + 1).to_string(),
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let score = vec![5,4,3,2,1];
        let expected = vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"];

        assert_eq!(Solution::find_relative_ranks(score), expected);
    }

    #[test]
    fn example2() {
        let score = vec![10,3,8,9,4];
        let expected = vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"];

        assert_eq!(Solution::find_relative_ranks(score), expected);
    }
}