struct Solution;
impl Solution {
    fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let (mut one_before, mut two_before) = (2, 1);

        for _ in 3..=n {
            let curr = one_before + two_before;
            two_before = one_before;
            one_before = curr;
        }

        one_before
    }

    fn climb_stairs_effective(n: i32) -> i32 {
        let mut memo = vec![-1; (n + 1) as usize];
        Self::count_ways(n, &mut memo)
    }

    fn count_ways(steps_remaining: i32, memo: &mut Vec<i32>) -> i32 {
        if steps_remaining <= 1 {
            return 1;
        }
        if memo[steps_remaining as usize] != -1 {
            return memo[steps_remaining as usize];
        }

        memo[steps_remaining as usize] = Self::count_ways(steps_remaining - 1, memo)
            + Self::count_ways(steps_remaining - 2, memo);
        memo[steps_remaining as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 2;
        let expected = 2;

        assert_eq!(Solution::climb_stairs(n), expected);
    }

    #[test]
    fn example2() {
        let n = 3;
        let expected = 3;

        assert_eq!(Solution::climb_stairs(n), expected);
    }

    #[test]
    fn example3() {
        let n = 2;
        let expected = 2;

        assert_eq!(Solution::climb_stairs_effective(n), expected);
    }

    #[test]
    fn example4() {
        let n = 3;
        let expected = 3;

        assert_eq!(Solution::climb_stairs_effective(n), expected);
    }
}