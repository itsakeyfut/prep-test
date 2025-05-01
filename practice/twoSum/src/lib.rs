//! Example1:
//!   Input: nums = [2,7,11,15], target = 9
//!   Output: [0,1]
//!   Explanation: Because nums[0] + nums[1] == 9, we return [0. 1].
//! 
//! Example2:
//!   Input: nums = [3,2,4], target = 6
//!   Output: [1,2]
//! 
//! Example3:
//!   Input: nums = [3,3], target = 6
//!   Output: [0,1]
//! 
//! 
//! Constraints:
//!   2 <= nums.length <= 104
//!   -109 <= nums[i] <= 109
//!   -109 <= target <= 109
//!   Only one valid answer exists.

struct Solution;

impl Solution {
    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // ハッシュマップを用意
        let mut map = std::collections::HashMap::new(); // Space: O(n)

        // 配列を一度走査
        for (idx, &num) in nums.iter().enumerate() { // Time: O(n)
            let complement = target - num;

            // 差分がすでにマップに存在するかを確認
            if let Some(&complement_idx) = map.get(&complement) { // Time: O(1)
                return vec![complement_idx as i32, idx as i32];
            }

            // 現在の値とインデックスをマップに追加
            map.insert(num, idx); // Time: O(1)
        }

        // 問題の制約から、常に解が存在するとされるので、この行には到達しないはず
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0,1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![3,2,4];
        let target = 6;
        let expected = vec![1,2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![3,3];
        let target = 6;
        let expected = vec![0,1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}