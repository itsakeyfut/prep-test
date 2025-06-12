struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnts = std::collections::HashMap::new();
        let mut result = Vec::new();

        for num in nums1 {
            *cnts.entry(num).or_insert(0) += 1;
        }

        for num in nums2 {
            if let Some(cnt) = cnts.get_mut(&num) {
                result.push(num);
                *cnt -= 1;
                if *cnt == 0 {
                    cnts.remove(&num);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1,2,2,1];
        let nums2 = vec![2,2];
        let expected = vec![2,2];

        assert_eq!(Solution::intersect(nums1, nums2), expected);
    }

    #[test]
    fn example2() {
        let nums1 = vec![4,9,5];
        let nums2 = vec![9,4,9,8,4];
        let expected = vec![9,4];

        assert_eq!(Solution::intersect(nums1, nums2), expected);
    }
}