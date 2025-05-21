struct Solution;
impl Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = Vec::new();
        if num_rows == 0 {
            return triangle;
        }

        triangle.push(vec![1]);

        for _ in 1..num_rows {
            let prev_row = triangle.last().unwrap();
            let mut new_row = vec![1];

            for j in 1..prev_row.len() {
                new_row.push(prev_row[j-1] + prev_row[j]);
            }

            new_row.push(1);
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
            vec![1,4,6,4,1],
        ];
        assert_eq!(Solution::generate(num_rows), expected);
    }
}