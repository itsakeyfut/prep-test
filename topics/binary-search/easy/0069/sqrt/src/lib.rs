struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut left = 1;
        let mut right = x / 2;

        while left <= right {
            let mid = (left + right) / 2;
            let square = mid.checked_mul(mid);

            match square {
                Some(val) if val == x => return mid,
                Some(val) if val < x => left = mid + 1,
                _ => right = mid - 1,
            }
        }

        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let x = 4;
        let expected = 2;

        assert_eq!(Solution::my_sqrt(x), expected);
    }

    #[test]
    fn example2() {
        let x = 8;
        let expected = 2;

        assert_eq!(Solution::my_sqrt(x), expected);
    }
}