struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; (n + 1) as usize];
        for i in 1..=n as usize {
            ans[i] = ans[i >> 1] + (i & 1) as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 2;
        let expected = vec![0,1,1];

        assert_eq!(Solution::count_bits(n), expected);
    }

    #[test]
    fn example2() {
        let n = 5;
        let expected = vec![0,1,1,2,1,2];

        assert_eq!(Solution::count_bits(n), expected);
    }
}