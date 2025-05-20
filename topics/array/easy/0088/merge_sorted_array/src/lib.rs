#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut k = (m + n - 1) as usize;
        let mut i = (m - 1) as i32;
        let mut j = (n - 1) as i32;

        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[k] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k] = nums2[j as usize];
                j -= 1;
            }
            k = k.wrapping_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;
        assert_eq!(Solution::merge(&mut nums1, m, &mut nums2, n), ());
    }

    #[test]
    fn example2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        assert_eq!(Solution::merge(&mut nums1, m, &mut nums2, n), ());
    }

    #[test]
    fn example3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        assert_eq!(Solution::merge(&mut nums1, m, &mut nums2, n), ());
    }
}
