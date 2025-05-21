struct Solution;
impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            max_area = max_area.max(h * w);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let expected = 49;
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let height = vec![1,1];
        let expected = 1;
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }
}