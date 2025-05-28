struct Solution;
impl Solution {
    fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![0,1,0,3,12];
        let expected = vec![1,3,12,0,0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);

    }

    #[test]
    fn example2() {
        let mut nums = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}