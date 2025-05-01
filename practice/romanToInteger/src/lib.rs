//! Example1:
//!   Input: s = "III"
//!   Output: 3
//!   Explanation: III = 3.
//! 
//! Example2:
//!   Input: s = "LVIII"
//!   Output: 58
//!   Explanation: L = 50, V = 5, III = 3
//! 
//! Example3:
//!   Input: s = "MCMXCIV"
//!   Output: 1994
//!   Explanation: M = 1000, CM = 900, XC = 90, and IV = 4
//! 
//! 
//! Constraints:
//! 　1 <= s.length <= 15
//! 　s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
//! 　It is guaranteed that s is a valid roman numeral in the range [1, 3999].


struct Solution;

impl Solution {
    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    pub fn roman_to_int(s: String) -> i32 {
        let s_replaced = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_replaced.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            }
        }).sum()
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    pub fn roman_to_int2(s: String) -> i32 {
        let mut result = 0;
        let mut prev_val = 0;

        // Process right-to-left for easier handling of subtraction cases
        for c in s.chars().rev() {
            let curr_val = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            // If current value is greater than or equal to previous.
            // add it; otherwise subtract it (handling cases like IV, IX, etc.)
            if curr_val >= prev_val {
                result += curr_val;
            } else {
                result -= curr_val;
            }

            prev_val = curr_val;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Simpler Solution
    #[test]
    fn example1() {
        let s = "III".to_string();
        let expected = 3;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn example2() {
        let s = "LVIII".to_string();
        let expected = 58;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn example3() {
        let s = "MCMXCIV".to_string();
        let expected = 1994;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    // Effective Solution
    #[test]
    fn example4() {
        let s = "III".to_string();
        let expected = 3;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn example5() {
        let s = "LVIII".to_string();
        let expected = 58;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn example6() {
        let s = "MCMXCIV".to_string();
        let expected = 1994;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
}