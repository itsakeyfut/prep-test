struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: std::collections::HashSet<_> = nums1.into_iter().collect();
        let set2: std::collections::HashSet<_> = nums2.into_iter().collect();
        set1.intersection(&set2).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1,2,2,1];
        let nums2 = vec![2,2];
        let expected = vec![2];

        assert_eq!(Solution::intersection(nums1, nums2), expected);
    }

    #[test]
    fn example2() {
        let nums1 = vec![4,9,5];
        let nums2 = vec![2,2];
        let expected = vec![2];

        assert_eq!(Solution::intersection(nums1, nums2), expected);
    }
}