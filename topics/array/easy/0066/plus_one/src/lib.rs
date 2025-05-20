#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            match *digit == 9 {
                true => *digit = 0,
                false => {
                    *digit += 1;
                    return digits;
                }
            }
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let digits = vec![1,2,3];
        let expected = vec![1,2,4];
        assert_eq!(Solution::plus_one(digits), expected);
    }

    #[test]
    fn example2() {
        let digits = vec![4,3,2,1];
        let expected = vec![4,3,2,2];
        assert_eq!(Solution::plus_one(digits), expected);
    }

    #[test]
    fn example3() {
        let digits = vec![9];
        let expected = vec![1,0];
        assert_eq!(Solution::plus_one(digits), expected);
    }
}