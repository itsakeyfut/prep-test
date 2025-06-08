struct Solution;
impl Solution {
    fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        for i in 1..=row_index as usize {
            row.push(0);
            for j in (1..=i).rev() {
                row[j] += row[j - 1];
            }
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let row_index = 3;
        let expected = vec![1,3,3,1];

        assert_eq!(Solution::get_row(row_index), expected);
    }

    #[test]
    fn example2() {
        let row_index = 0;
        let expected = vec![1];

        assert_eq!(Solution::get_row(row_index), expected);
    }

    #[test]
    fn example3() {
        let row_index = 1;
        let expected = vec![1,1];

        assert_eq!(Solution::get_row(row_index), expected);
    }
}