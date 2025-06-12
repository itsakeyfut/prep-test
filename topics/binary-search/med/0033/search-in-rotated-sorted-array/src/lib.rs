struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = (low + high) / 2;

            if nums[mid as usize] == target {
                return mid;
            }

            if nums[low as usize] <= nums[mid as usize] {
                if nums[low as usize] <= target && target < nums[mid as usize] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[high as usize] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        let expected = 4;

        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;
        let expected = -1;

        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        let target = 0;
        let expected = -1;

        assert_eq!(Solution::search(nums, target), expected);
    }
}