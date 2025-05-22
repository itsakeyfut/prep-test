struct Solution;
impl Solution {
    fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1];
        let mut prev: i64 = 1;

        for k in 1..=row_index {
            let next_val = prev * (row_index - k + 1) as i64 / k as i64;
            res.push(next_val as i32);
            prev = next_val;
        }
        res
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