struct Solution;
impl Solution {
    fn insersection(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums2.sort();
        let search_arr = Self::make_search_fn(&nums2);
        let mut seen = std::collections::HashSet::new();

        nums1.into_iter()
            .filter(|&x| search_arr(x) && seen.insert(x))
            .collect()
    }

    fn make_search_fn<'a>(arr: &'a[i32]) -> impl Fn(i32) -> bool + 'a {
        move |target: i32| Self::binary_search(arr, target)
    }

    fn binary_search(arr: &[i32], target: i32) -> bool {
        let mut lo: isize = 0;
        let mut hi: isize = arr.len() as isize - 1;

        while lo <= hi {
            let mid: isize = lo + (hi - lo) / 2;
            if arr[mid as usize] == target {
                return true;
            }
            else if arr[mid as usize] < target {
                lo = mid + 1;
            }
            else {
                hi = mid - 1;
            }
        }
        return false;
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

        assert_eq!(Solution::insersection(nums1, nums2), expected);
    }

    #[test]
    fn example2() {
        let nums1 = vec![4,9,5];
        let nums2 = vec![9,4,9,8,4];
        let expected = vec![4,9];

        assert_eq!(Solution::insersection(nums1, nums2), expected);
    }
}