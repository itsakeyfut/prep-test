struct Solution;
impl Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::new();
        if num_rows == 0 {
            return triangle;
        }

        triangle.push(vec![1]);

        for _ in 1..num_rows {
            let prev = triangle.last().unwrap();
            let new_row = std::iter::once(1)
                .chain((1..prev.len()).map(|j| prev[j - 1] + prev[j]))
                .chain(std::iter::once(1))
                .collect();
            triangle.push(new_row);
        }

        triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let num_rows = 5;
        let expected = vec![
            vec![1],
            vec![1,1],
            vec![1,2,1],
            vec![1,3,3,1],
            vec![1,4,6,4,1]
        ];

        assert_eq!(Solution::generate(num_rows), expected);
    }

    #[test]
    fn example2() {
        let num_rows = 1;
        let expected = vec![
            vec![1],
        ];

        assert_eq!(Solution::generate(num_rows), expected);
    }
}