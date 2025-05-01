//! Example1:
//!   Input: x = 121
//!   Output: true
//!   Explanation: 121 reads as 121 from left to right and from right to left.
//! 
//! Example2:
//!   Input: x = -121
//!   Output: false
//!   Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//! 
//! Example3:
//!   Input: x = 10
//!   Output: false
//!   Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//! 
//! 
//! Constraints:
//!   -2^31 <= x <= 2^31 - 1
//! 

struct Solution;

impl Solution {
    /// Time Complexity: O(log n)
    /// Space Complexity: O(log n)
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }


    /// Time Complexity: O(log n)
    /// Space Complexity: O(1)
    /// 
    /// x = 121 の場合
    /// ループ開始：original = 121, reversed = 0
    /// 　1回目：reversed = 0*10 + 121%10 = 1, original = 121/10 = 12
    /// 　2回目：reversed = 1*10 + 12%10 = 12, original = 12/10 = 1
    /// 　ループ終了：original = 1 は reversed = 12 より小さいので
    /// original == reversed は 1 == 12 なのでfalse
    /// original == reversed/10 は 1 == 12/10 すなわち 1 == 1 なのでtrue
    /// 最終的に true を返す
    /// 
    /// このアルゴリズムは奇数桁の回文数に対して上手く動作する
    /// 
    pub fn is_palindrome2(x: i32) -> bool {
        // Negative numbers can't be palindromes (due to the minus sign)
        if x < 0 {
            return false;
        }

        // Single digit numbers are always palindromes
        if x < 10 {
            return true;
        }

        // Numbers ending with 0 can't be palindrome unless it's 0 itself
        if x % 10 == 0 && x != 0 {
            return false;
        }

        let mut original = x;
        let mut reversed = 0;

        // Reverse half of the number to avoid overflow
        while original > reversed {
            reversed = reversed * 10 + original % 10;
            original /= 10;
        }
        // For even number of digits: original == reversed
        // For odd number of digits: original == reversed/10 (to ignore middle digit)
        original == reversed || original == reversed / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simpler Solution
    #[test]
    fn example1() {
        let x = 121;
        let expected = true;
        assert_eq!(Solution::is_palindrome(x), expected);
    }

    #[test]
    fn example2() {
        let x = -121;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }

    #[test]
    fn example3() {
        let x = 10;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }

    // Mathematical Solution
    #[test]
    fn example4() {
        let x = 121;
        let expected = true;
        assert_eq!(Solution::is_palindrome2(x), expected);
    }

    #[test]
    fn example5() {
        let x = -121;
        let expected = false;
        assert_eq!(Solution::is_palindrome2(x), expected);
    }

    #[test]
    fn example6() {
        let x = 10;
        let expected = false;
        assert_eq!(Solution::is_palindrome2(x), expected);
    }
}
