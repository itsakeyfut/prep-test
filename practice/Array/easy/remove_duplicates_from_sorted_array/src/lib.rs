//! Example 1:
//!
//! Input: nums = [1,1,2]
//! Output: 2, nums = [1,2,_]
//! Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//! 
//! Example 2:
//! 
//! Input: nums = [0,0,1,1,1,2,2,3,3,4]
//! Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
//! Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//! 

struct Solution {}

impl Solution {
    pub fn remove_dupilicates(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => return 0,
            false => {
                let mut write_idx = 0;
                for read_idx in 1..nums.len() {
                    if nums[write_idx] != nums[read_idx] {
                        write_idx += 1;
                        nums[write_idx] = nums[read_idx];
                    }
                }
                (write_idx + 1) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1, 1, 2];
        let expected = 2;
        assert_eq!(Solution::remove_dupilicates(&mut nums), expected);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let expected = 5;
        assert_eq!(Solution::remove_dupilicates(&mut nums), expected);
    }
}